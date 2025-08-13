use std::f64::consts::PI;

const EARTH_RADIUS_KM: f64 = 6371.0;
const EARTH_RADIUS_M: f64 = 6371000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, &'static str> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err("Latitude must be between -90 and 90 degrees");
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err("Longitude must be between -180 and 180 degrees");
        }
        
        Ok(Coordinate { latitude, longitude })
    }

    pub fn to_radians(&self) -> (f64, f64) {
        (self.latitude.to_radians(), self.longitude.to_radians())
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub north: f64,
    pub south: f64,
    pub east: f64,
    pub west: f64,
}

impl BoundingBox {
    pub fn new(north: f64, south: f64, east: f64, west: f64) -> Result<Self, &'static str> {
        if north <= south {
            return Err("North must be greater than south");
        }
        if east <= west {
            return Err("East must be greater than west");
        }
        
        Ok(BoundingBox { north, south, east, west })
    }

    pub fn contains(&self, coord: &Coordinate) -> bool {
        coord.latitude <= self.north
            && coord.latitude >= self.south
            && coord.longitude <= self.east
            && coord.longitude >= self.west
    }

    pub fn center(&self) -> Coordinate {
        let lat = (self.north + self.south) / 2.0;
        let lng = (self.east + self.west) / 2.0;
        Coordinate::new(lat, lng).unwrap()
    }

    pub fn expand(&mut self, coord: &Coordinate) {
        self.north = self.north.max(coord.latitude);
        self.south = self.south.min(coord.latitude);
        self.east = self.east.max(coord.longitude);
        self.west = self.west.min(coord.longitude);
    }
}

pub fn haversine_distance(coord1: &Coordinate, coord2: &Coordinate) -> f64 {
    let (lat1_rad, lng1_rad) = coord1.to_radians();
    let (lat2_rad, lng2_rad) = coord2.to_radians();

    let dlat = lat2_rad - lat1_rad;
    let dlng = lng2_rad - lng1_rad;

    let a = (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_M * c
}

pub fn bearing(from: &Coordinate, to: &Coordinate) -> f64 {
    let (lat1_rad, lng1_rad) = from.to_radians();
    let (lat2_rad, lng2_rad) = to.to_radians();

    let dlng = lng2_rad - lng1_rad;

    let y = dlng.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * dlng.cos();

    let bearing_rad = y.atan2(x);
    (bearing_rad.to_degrees() + 360.0) % 360.0
}

pub fn destination