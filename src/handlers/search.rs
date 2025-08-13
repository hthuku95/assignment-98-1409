use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

use crate::{
    error::AppError,
    models::{Location, SearchResult},
    services::geocoding::GeocodingService,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub radius: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: usize,
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct GeocodeQuery {
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct GeocodeResponse {
    pub location: Location,
    pub formatted_address: String,
    pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct ReverseGeocodeQuery {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize)]
pub struct ReverseGeocodeResponse {
    pub address: String,
    pub components: HashMap<String, String>,
    pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct AutocompleteQuery {
    pub input: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub radius: Option<u32>,
    pub types: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AutocompleteResponse {
    pub predictions: Vec<AutocompletePrediction>,
}

#[derive(Debug, Serialize)]
pub struct AutocompletePrediction {
    pub place_id: String,
    pub description: String,
    pub structured_formatting: StructuredFormatting,
    pub distance_meters: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct StructuredFormatting {
    pub main_text: String,
    pub secondary_text: String,
}

pub async fn search_locations(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, AppError> {
    info!("Searching for locations: {}", params.q);

    if params.q.trim().is_empty() {
        return Err(AppError::BadRequest("Search query cannot be empty".to_string()));
    }

    let limit = params.limit.unwrap_or(10).min(100);
    let radius = params.radius.unwrap_or(10000);

    let geocoding_service = GeocodingService::new(&state.config.google_api_key);
    
    let results = match (params.lat, params.lng) {
        (Some(lat), Some(lng)) => {
            geocoding_service
                .search_nearby(&params.q, lat, lng, radius, limit)
                .await
                .map_err(|e| {
                    error!("Failed to search nearby locations: {}", e);
                    AppError::InternalServerError("Failed to search locations".to_string())
                })?
        }
        _ => {
            geocoding_service
                .text_search(&params.q, limit)
                .await
                .map_err(|e| {
                    error!("Failed to perform text search: {}", e);
                    AppError::InternalServerError("Failed to search locations".to_string())
                })?
        }
    };

    let response = SearchResponse {
        total_count: results.len(),
        query: params.q.clone(),
        results,
    };

    info!("Found {} results for query: {}", response.total_count, params.q);
    Ok(Json(response))
}

pub async fn geocode_address(
    Query(