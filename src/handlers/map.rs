use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Location, Route, Place};
use crate::services::geocoding::GeocodingService;
use crate::services::routing::RoutingService;
use crate::services::places::PlacesService;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub radius: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct RouteQuery {
    pub origin: String,
    pub destination: String,
    pub waypoints: Option<Vec<String>>,
    pub travel_mode: Option<String>,
    pub avoid_tolls: Option<bool>,
    pub avoid_highways: Option<bool>,
}

#[derive(Deserialize)]
pub struct NearbyQuery {
    pub lat: f64,
    pub lng: f64,
    pub radius: Option<i32>,
    pub place_type: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<Place>,
    pub status: String,
}

#[derive(Serialize)]
pub struct RouteResponse {
    pub routes: Vec<Route>,
    pub status: String,
    pub distance: Option<f64>,
    pub duration: Option<i32>,
}

#[derive(Serialize)]
pub struct NearbyResponse {
    pub places: Vec<Place>,
    pub status: String,
}

#[derive(Serialize)]
pub struct GeocodeResponse {
    pub results: Vec<Location>,
    pub status: String,
}

pub async fn search_places(
    query: web::Query<SearchQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let places_service = PlacesService::new(pool.get_ref());
    
    let limit = query.limit.unwrap_or(20).min(50);
    let radius = query.radius.unwrap_or(5000);
    
    let results = if let (Some(lat), Some(lng)) = (query.lat, query.lng) {
        places_service
            .search_nearby(&query.q, lat, lng, radius, limit)
            .await?
    } else {
        places_service
            .search_global(&query.q, limit)
            .await?
    };

    Ok(HttpResponse::Ok().json(SearchResponse {
        results,
        status: "OK".to_string(),
    }))
}

pub async fn get_directions(
    query: web::Query<RouteQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let routing_service = RoutingService::new(pool.get_ref());
    let geocoding_service = GeocodingService::new();

    // Geocode origin and destination
    let origin_location = geocoding_service.geocode(&query.origin).await?;
    let destination_location = geocoding_service.geocode(&query.destination).await?;

    if origin_location.is_empty() || destination_location.is_empty() {
        return Ok(HttpResponse::BadRequest().json(RouteResponse {
            routes: vec![],
            status: "ZERO_RESULTS".to_string(),
            distance: None,
            duration: None,
        }));
    }

    let origin = &origin_location[0];
    let destination = &destination_location[0];

    // Handle waypoints if provided
    let mut waypoint_locations = Vec::new();
    if let Some(waypoints) = &query.waypoints {
        for waypoint in waypoints {
            let locations = geocoding_service.geocode(waypoint).await?;
            if !locations.is_empty() {
                waypoint_locations.push(locations[0].