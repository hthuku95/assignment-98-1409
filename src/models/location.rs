use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, LocationError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(LocationError::InvalidLatitude(latitude));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(LocationError::InvalidLongitude(longitude));
        }
        
        Ok(Coordinate { latitude, longitude })
    }

    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        
        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lng = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
        
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS_KM * c
    }

    pub fn bearing_to(&self, other: &Coordinate) -> f64 {
        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lng = (other.longitude - self.longitude).to_radians();

        let y = delta_lng.sin() * lat2_rad.cos();
        let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lng.cos();

        let bearing_rad = y.atan2(x);
        (bearing_rad.to_degrees() + 360.0) % 360.0
    }

    pub fn midpoint_to(&self, other: &Coordinate) -> Coordinate {
        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lng = (other.longitude - self.longitude).to_radians();

        let bx = lat2_rad.cos() * delta_lng.cos();
        let by = lat2_rad.cos() * delta_lng.sin();

        let lat3_rad = (lat1_rad + lat2_rad.cos() * (lat1_rad.cos() + bx)).atan2((lat1_rad.sin() + lat2_rad.sin()).hypot(lat1_rad.cos() + bx));
        let lng3_rad = self.longitude.to_radians() + by.atan2(lat1_rad.cos() + bx);

        Coordinate {
            latitude: lat3_rad.to_degrees(),
            longitude: lng3_rad.to_degrees(),
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.6}, {:.6})", self.latitude, self.longitude)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub southwest: Coordinate,
    pub northeast: Coordinate,
}

impl BoundingBox {
    pub fn new(southwest: Coordinate, northeast: Coordinate) -> Result<Self, LocationError> {
        if southwest.latitude > northeast.latitude {
            return Err(LocationError::InvalidBoundingBox("Southwest latitude cannot be greater than northeast latitude".to_string()));