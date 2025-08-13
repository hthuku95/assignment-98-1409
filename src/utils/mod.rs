//! Utility functions for the Google Maps clone application
//! 
//! This module provides common utility functions used throughout the application
//! including coordinate transformations, distance calculations, and data validation.

pub mod coordinates;
pub mod distance;
pub mod validation;
pub mod geohash;
pub mod cache;
pub mod config;
pub mod error;
pub mod time;

pub use coordinates::*;
pub use distance::*;
pub use validation::*;
pub use geohash::*;
pub use cache::*;
pub use config::*;
pub use error::*;
pub use time::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Common result type used throughout the application
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Configuration constants
pub const DEFAULT_ZOOM_LEVEL: u8 = 10;
pub const MAX_ZOOM_LEVEL: u8 = 18;
pub const MIN_ZOOM_LEVEL: u8 = 1;
pub const TILE_SIZE: u32 = 256;
pub const EARTH_RADIUS_KM: f64 = 6371.0;
pub const EARTH_RADIUS_M: f64 = 6371000.0;

/// Common data structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox {
    pub north: f64,
    pub south: f64,
    pub east: f64,
    pub west: f64,
}

impl BoundingBox {
    pub fn new(north: f64, south: f64, east: f64, west: f64) -> Self {
        Self { north, south, east, west }
    }

    pub fn contains(&self, lat: f64, lng: f64) -> bool {
        lat >= self.south && lat <= self.north && lng >= self.west && lng <= self.east
    }

    pub fn center(&self) -> Coordinate {
        Coordinate::new(
            (self.north + self.south) / 2.0,
            (self.east + self.west) / 2.0,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileCoordinate {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl TileCoordinate {
    pub fn new(x: u32, y: u32, z: u8) -> Self {
        Self { x, y, z }
    }
}

/// Utility functions for common operations
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn normalize_longitude(lng: f64) -> f64 {
    let mut normalized = lng % 360.0;
    if normalized > 180.0 {
        normalized -= 360.0;
    } else if normalized < -180.0 {
        normalized += 360.0;
    }
    normalized
}

pub fn normalize_latitude(lat: f64) -> f64 {
    clamp(lat, -90.0, 90.0)
}

/// Convert degrees to radians
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// Convert radians to degrees
pub fn rad_to_deg(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

/// Generate a unique ID
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}

/// Parse query parameters from URL
pub