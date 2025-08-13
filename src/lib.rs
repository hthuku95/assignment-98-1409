//! Google Maps Clone - A comprehensive mapping application built in Rust
//! 
//! This library provides core functionality for a Google Maps-like application,
//! including map rendering, routing, geocoding, and user interface components.

pub mod core;
pub mod ui;
pub mod network;
pub mod utils;

pub use core::{
    map::{Map, MapRenderer, Viewport, ZoomLevel},
    location::{Coordinate, Location, BoundingBox},
    routing::{Route, RouteCalculator, RouteSegment, TravelMode},
    search::{SearchEngine, SearchResult, PlaceType},
    tiles::{TileManager, TileProvider, TileCache},
};

pub use ui::{
    components::{MapWidget, SearchBar, NavigationPanel, InfoWindow},
    events::{MapEvent, UserInteraction, EventHandler},
    styles::{MapStyle, Theme, ColorScheme},
};

pub use network::{
    api::{ApiClient, ApiError, ApiResponse},
    geocoding::{GeocodingService, ReverseGeocodingService},
    directions::{DirectionsService, DirectionsRequest, DirectionsResponse},
};

pub use utils::{
    geometry::{distance_between, bearing, interpolate_point},
    conversion::{lat_lng_to_tile, tile_to_lat_lng, mercator_projection},
    cache::{CacheManager, CacheEntry, CachePolicy},
    config::{AppConfig, MapConfig, NetworkConfig},
};

use std::error::Error;
use std::fmt;

/// Main application error type
#[derive(Debug)]
pub enum MapsError {
    /// Network-related errors
    Network(String),
    /// Parsing or data format errors
    Parse(String),
    /// Geographic calculation errors
    Geographic(String),
    /// UI rendering errors
    Rendering(String),
    /// Configuration errors
    Config(String),
    /// Cache operation errors
    Cache(String),
}

impl fmt::Display for MapsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapsError::Network(msg) => write!(f, "Network error: {}", msg),
            MapsError::Parse(msg) => write!(f, "Parse error: {}", msg),
            MapsError::Geographic(msg) => write!(f, "Geographic error: {}", msg),
            MapsError::Rendering(msg) => write!(f, "Rendering error: {}", msg),
            MapsError::Config(msg) => write!(f, "Configuration error: {}", msg),
            MapsError::Cache(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl Error for MapsError {}

/// Result type used throughout the application
pub type MapsResult<T> = Result<T, MapsError>;

/// Application version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str = "Maps Clone";

/// Default configuration constants
pub mod constants {
    /// Default map center coordinates (San Francisco)
    pub const DEFAULT_LAT: f64 = 37.7749;
    pub const DEFAULT_LNG: f64 = -122.4194;
    
    /// Default zoom level
    pub const DEFAULT_ZOOM: u8 = 12;
    
    /// Tile server configuration
    pub const TILE_SIZE: u32 = 256;
    pub const MAX_ZOOM: u8 = 18;
    pub const MIN_ZOOM: u8 = 1;
    
    /// Cache configuration
    pub const DEFAULT_CACHE_SIZE: usize = 100 * 1024 * 1024; // 100MB
    pub const CACHE_EXPIRY_HOURS: u64 = 24;
    
    /// Network timeouts (in seconds)
    pub const REQUEST_TIMEOUT: u64 = 30;
    pub const CONNECT_TIMEOUT: u64 = 10;
    
    /// UI configuration
    pub const SEARCH_DEBOUNCE_MS: u64 = 300;
    pub const ANIMATION_DURATION_MS: u64 = 250;
}