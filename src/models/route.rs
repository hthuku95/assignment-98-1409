use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        let earth_radius = 6371000.0; // meters
        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        earth_radius * c
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSegment {
    pub start: Coordinate,
    pub end: Coordinate,
    pub distance: f64,
    pub duration: i32, // seconds
    pub instruction: String,
    pub street_name: Option<String>,
    pub maneuver: ManeuverType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManeuverType {
    Start,
    Straight,
    TurnLeft,
    TurnRight,
    TurnSlightLeft,
    TurnSlightRight,
    TurnSharpLeft,
    TurnSharpRight,
    UTurn,
    Merge,
    RampLeft,
    RampRight,
    Fork,
    Roundabout,
    Exit,
    Arrive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub origin: Coordinate,
    pub destination: Coordinate,
    pub waypoints: Vec<Coordinate>,
    pub segments: Vec<RouteSegment>,
    pub total_distance: f64,
    pub total_duration: i32,
    pub route_type: RouteType,
    pub created_at: DateTime<Utc>,
    pub traffic_info: Option<TrafficInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteType {
    Fastest,
    Shortest,
    Balanced,
    AvoidTolls,
    AvoidHighways,
    Walking,
    Cycling,
    Transit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficInfo {
    pub current_conditions: TrafficCondition,
    pub delays: Vec<TrafficDelay>,
    pub alternative_routes_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrafficCondition {
    Light,
    Moderate,
    Heavy,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficDelay {
    pub segment_index: usize,
    pub delay_seconds: i32,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationInstruction {
    pub id: String,
    pub segment_index: usize,
    pub distance_to_instruction: f64,
    pub instruction_text: String,
    pub voice_instruction: Option<String>,
    pub icon: String,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationSession {
    pub id: String,
    pub route: Route,
    pub current_position: Coordinate