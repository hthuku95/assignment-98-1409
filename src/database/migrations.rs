use std::collections::HashMap;
use sqlx::{Pool, Postgres, Row};
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct Migration {
    pub version: i32,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

pub struct MigrationRunner {
    pool: Pool<Postgres>,
    migrations: Vec<Migration>,
}

impl MigrationRunner {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let mut runner = Self {
            pool,
            migrations: Vec::new(),
        };
        runner.register_migrations();
        runner
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        info!("Starting database migrations");
        
        self.create_migration_table().await?;
        let applied_migrations = self.get_applied_migrations().await?;
        
        for migration in &self.migrations {
            if !applied_migrations.contains(&migration.version) {
                info!("Applying migration {}: {}", migration.version, migration.name);
                self.apply_migration(migration).await?;
            } else {
                info!("Migration {} already applied", migration.version);
            }
        }
        
        info!("Database migrations completed");
        Ok(())
    }

    pub async fn rollback_migration(&self, target_version: i32) -> Result<(), sqlx::Error> {
        info!("Rolling back migrations to version {}", target_version);
        
        let applied_migrations = self.get_applied_migrations().await?;
        let mut migrations_to_rollback: Vec<&Migration> = self.migrations
            .iter()
            .filter(|m| m.version > target_version && applied_migrations.contains(&m.version))
            .collect();
        
        migrations_to_rollback.sort_by(|a, b| b.version.cmp(&a.version));
        
        for migration in migrations_to_rollback {
            info!("Rolling back migration {}: {}", migration.version, migration.name);
            self.rollback_migration_version(migration).await?;
        }
        
        info!("Migration rollback completed");
        Ok(())
    }

    async fn create_migration_table(&self) -> Result<(), sqlx::Error> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#;
        
        sqlx::query(query).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_applied_migrations(&self) -> Result<Vec<i32>, sqlx::Error> {
        let rows = sqlx::query("SELECT version FROM migrations ORDER BY version")
            .fetch_all(&self.pool)
            .await?;
        
        Ok(rows.iter().map(|row| row.get("version")).collect())
    }

    async fn apply_migration(&self, migration: &Migration) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        match sqlx::query(&migration.up_sql).execute(&mut *tx).await {
            Ok(_) => {
                sqlx::query("INSERT INTO migrations (version, name) VALUES ($1, $2)")
                    .bind(migration.version)
                    .bind(&migration.name)
                    .execute(&mut *tx)
                    .await?;
                
                tx.commit().await?;
                info!("Successfully applied migration {}", migration.version);
                Ok(())
            }
            Err(e) => {
                tx.rollback().await?;
                error!("Failed to apply migration {}: {}", migration.version, e);
                Err(e)
            }
        }
    }

    async fn rollback_migration_version(&self, migration: &Migration) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        match sqlx::query(&migration.down_