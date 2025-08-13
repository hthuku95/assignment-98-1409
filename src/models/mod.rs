//! Data models module declaration
//!
//! This module contains all the data structures and models used throughout
//! the Google Maps clone application.

pub mod coordinate;
pub mod location;
pub mod route;
pub mod search;
pub mod user;
pub mod map_tile;
pub mod poi;
pub mod traffic;
pub mod directions;
pub mod geocoding;

// Re-export commonly used types for convenience
pub use coordinate::Coordinate;
pub use location::{Location, LocationType};
pub use route::{Route, RouteSegment, TravelMode};
pub use search::{SearchQuery, SearchResult, SearchFilter};
pub use user::{User, UserPreferences};
pub use map_tile::{MapTile, TileCoordinate, ZoomLevel};
pub use poi::{PointOfInterest, POICategory};
pub use traffic::{TrafficInfo, TrafficLevel};
pub use directions::{DirectionsRequest, DirectionsResponse, Step};
pub use geocoding::{GeocodingRequest, GeocodingResponse, AddressComponent};

/// Common result type used throughout the application
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Geographic bounds representing a rectangular area
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub northeast: Coordinate,
    pub southwest: Coordinate,
}

impl Bounds {
    pub fn new(northeast: Coordinate, southwest: Coordinate) -> Self {
        Self {
            northeast,
            southwest,
        }
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        coordinate.latitude <= self.northeast.latitude
            && coordinate.latitude >= self.southwest.latitude
            && coordinate.longitude <= self.northeast.longitude
            && coordinate.longitude >= self.southwest.longitude
    }

    pub fn center(&self) -> Coordinate {
        Coordinate {
            latitude: (self.northeast.latitude + self.southwest.latitude) / 2.0,
            longitude: (self.northeast.longitude + self.southwest.longitude) / 2.0,
        }
    }
}

/// Distance measurement with unit
#[derive(Debug, Clone, PartialEq)]
pub struct Distance {
    pub value: f64,
    pub unit: DistanceUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DistanceUnit {
    Meters,
    Kilometers,
    Miles,
    Feet,
}

impl Distance {
    pub fn new(value: f64, unit: DistanceUnit) -> Self {
        Self { value, unit }
    }

    pub fn to_meters(&self) -> f64 {
        match self.unit {
            DistanceUnit::Meters => self.value,
            DistanceUnit::Kilometers => self.value * 1000.0,
            DistanceUnit::Miles => self.value * 1609.344,
            DistanceUnit::Feet => self.value * 0.3048,
        }
    }
}

/// Duration measurement
#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub seconds: u64,
}

impl Duration {
    pub fn new(seconds: u64) -> Self {
        Self { seconds }
    }

    pub fn from_minutes(minutes: u64) -> Self {
        Self {
            seconds: minutes * 60,
        }
    }

    pub fn from_hours(hours: u64) -> Self {
        Self {
            seconds: hours * 3600,
        }
    }

    pub fn to_minutes(&self) -> u64 {
        self.seconds / 60
    }

    pub fn to_hours(&self) -> f64 {
        self.seconds as f64 / 3600.0
    }
}