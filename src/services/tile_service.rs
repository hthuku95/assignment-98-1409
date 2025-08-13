use crate::models::tile::{Tile, TileCoordinate, TileFormat, TileData};
use crate::config::Config;
use crate::error::{Result, MapError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use tokio::fs;
use tokio::time::{Duration, Instant};
use image::{ImageBuffer, RgbImage, Rgb};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMetadata {
    pub coordinate: TileCoordinate,
    pub format: TileFormat,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub size_bytes: u64,
    pub cache_key: String,
}

#[derive(Debug, Clone)]
pub struct CachedTile {
    pub metadata: TileMetadata,
    pub data: TileData,
    pub expires_at: Instant,
}

pub struct TileService {
    config: Arc<Config>,
    memory_cache: Arc<RwLock<HashMap<String, CachedTile>>>,
    disk_cache_path: PathBuf,
    cache_stats: Arc<RwLock<CacheStats>>,
}

#[derive(Debug, Default)]
struct CacheStats {
    hits: u64,
    misses: u64,
    evictions: u64,
    disk_reads: u64,
    disk_writes: u64,
}

impl TileService {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let disk_cache_path = PathBuf::from(&config.tile_cache_dir);
        
        Ok(Self {
            config,
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            disk_cache_path,
            cache_stats: Arc::new(RwLock::new(CacheStats::default())),
        })
    }

    pub async fn initialize(&self) -> Result<()> {
        // Create cache directory if it doesn't exist
        if !self.disk_cache_path.exists() {
            fs::create_dir_all(&self.disk_cache_path).await
                .map_err(|e| MapError::IoError(format!("Failed to create cache directory: {}", e)))?;
        }

        // Load existing tiles from disk cache
        self.load_disk_cache().await?;

        info!("Tile service initialized with cache directory: {:?}", self.disk_cache_path);
        Ok(())
    }

    pub async fn get_tile(&self, coordinate: &TileCoordinate, format: TileFormat) -> Result<Tile> {
        let cache_key = self.generate_cache_key(coordinate, format);
        
        // Try memory cache first
        if let Some(cached_tile) = self.get_from_memory_cache(&cache_key).await {
            if cached_tile.expires_at > Instant::now() {
                self.update_cache_stats(true, false).await;
                return Ok(Tile {
                    coordinate: coordinate.clone(),
                    format,
                    data: cached_tile.data,
                    metadata: Some(cached_tile.metadata),
                });
            } else {
                // Remove expired tile
                self.remove_from_memory_cache(&cache_key).await;
            }
        }

        // Try disk cache
        if let Some(tile) = self.get_from_disk_cache(&cache_key).await? {
            // Add back to memory cache
            self.add_to_memory_cache(cache_key.clone(), &tile).await;
            self.update_cache_stats(true, true).await;
            return Ok(tile);
        }

        // Generate new tile
        self.update_cache_stats(false, false).await;
        let tile = self.generate_tile(coordinate, format).await?