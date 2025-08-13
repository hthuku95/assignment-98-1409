use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TileCoord {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PixelCoord {
    pub x: f64,
    pub y: f64,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }

    pub fn is_valid(&self) -> bool {
        self.lat >= -90.0 && self.lat <= 90.0 && self.lng >= -180.0 && self.lng <= 180.0
    }

    pub fn to_tile_coord(&self, zoom: u8) -> TileCoord {
        let lat_rad = self.lat.to_radians();
        let n = 2_f64.powi(zoom as i32);
        
        let x = ((self.lng + 180.0) / 360.0 * n).floor() as u32;
        let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * n).floor() as u32;
        
        TileCoord { x, y, z: zoom }
    }

    pub fn to_pixel_coord(&self, zoom: u8) -> PixelCoord {
        let lat_rad = self.lat.to_radians();
        let n = 2_f64.powi(zoom as i32);
        let tile_size = 256.0;
        
        let x = (self.lng + 180.0) / 360.0 * n * tile_size;
        let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * n * tile_size;
        
        PixelCoord { x, y }
    }

    pub fn distance_to(&self, other: &LatLng) -> f64 {
        const EARTH_RADIUS: f64 = 6371000.0; // meters
        
        let lat1_rad = self.lat.to_radians();
        let lat2_rad = other.lat.to_radians();
        let delta_lat = (other.lat - self.lat).to_radians();
        let delta_lng = (other.lng - self.lng).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) + 
                lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS * c
    }
}

impl TileCoord {
    pub fn new(x: u32, y: u32, z: u8) -> Self {
        Self { x, y, z }
    }

    pub fn to_lat_lng(&self) -> LatLng {
        let n = 2_f64.powi(self.z as i32);
        let lng = self.x as f64 / n * 360.0 - 180.0;
        let lat_rad = ((PI * (1.0 - 2.0 * self.y as f64 / n)).sinh()).atan();
        let lat = lat_rad.to_degrees();
        
        LatLng::new(lat, lng)
    }

    pub fn to_bounds(&self) -> (LatL