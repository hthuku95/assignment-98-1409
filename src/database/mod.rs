use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};
use std::env;
use anyhow::{Result, Context};

pub mod models;
pub mod queries;

pub use models::*;

#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable not set")?;
        
        let pool = PgPool::connect(&database_url)
            .await
            .context("Failed to connect to database")?;
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run database migrations")?;
        
        Ok(Database { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // Location operations
    pub async fn create_location(&self, location: &CreateLocation) -> Result<Location> {
        let row = sqlx::query!(
            r#"
            INSERT INTO locations (name, latitude, longitude, address, place_type, rating)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, latitude, longitude, address, place_type, rating, created_at, updated_at
            "#,
            location.name,
            location.latitude,
            location.longitude,
            location.address,
            location.place_type,
            location.rating
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create location")?;

        Ok(Location {
            id: row.id,
            name: row.name,
            latitude: row.latitude,
            longitude: row.longitude,
            address: row.address,
            place_type: row.place_type,
            rating: row.rating,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn get_location(&self, id: i32) -> Result<Option<Location>> {
        let row = sqlx::query!(
            "SELECT id, name, latitude, longitude, address, place_type, rating, created_at, updated_at FROM locations WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch location")?;

        Ok(row.map(|r| Location {
            id: r.id,
            name: r.name,
            latitude: r.latitude,
            longitude: r.longitude,
            address: r.address,
            place_type: r.place_type,
            rating: r.rating,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn search_locations(&self, query: &str, limit: i64) -> Result<Vec<Location>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, latitude, longitude, address, place_type, rating, created_at, updated_at
            FROM locations
            WHERE name ILIKE $1 OR address ILIKE $1
            ORDER BY name
            LIMIT $2
            "#,
            format!("%{}%", query),
            limit
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to search locations")?;

        Ok(rows.into_iter().map(|row| Location {
            id: row.id,
            name: row.name,
            latitude: row.latitude,
            longitude: row.longitude,
            address: row.address,
            place_type: row.place_type,
            rating: row.rating,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect())
    }

    pub async fn get_locations_in_bounds(&self, bounds: &MapBounds) -> Result<Vec<Location>> {
        let rows = sqlx::query!(