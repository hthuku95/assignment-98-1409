use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tokio::time::{timeout, Duration};
use log::{info, warn, error};
use crate::models::location::{Location, Coordinates};
use crate::config::Config;
use crate::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodingRequest {
    pub address: String,
    pub country: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodingResponse {
    pub coordinates: Coordinates,
    pub formatted_address: String,
    pub components: AddressComponents,
    pub place_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressComponents {
    pub street_number: Option<String>,
    pub route: Option<String>,
    pub locality: Option<String>,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenStreetMapResponse {
    pub lat: String,
    pub lon: String,
    pub display_name: String,
    pub address: Option<OSMAddress>,
    pub place_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OSMAddress {
    pub house_number: Option<String>,
    pub road: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postcode: Option<String>,
}

pub struct GeocodingService {
    client: Client,
    config: Config,
    cache: HashMap<String, GeocodingResponse>,
}

impl GeocodingService {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("RustMaps/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config,
            cache: HashMap::new(),
        }
    }

    pub async fn geocode(&mut self, request: &GeocodingRequest) -> Result<GeocodingResponse, ServiceError> {
        let cache_key = self.generate_cache_key(request);
        
        // Check cache first
        if let Some(cached_response) = self.cache.get(&cache_key) {
            info!("Returning cached geocoding result for: {}", request.address);
            return Ok(cached_response.clone());
        }

        // Perform geocoding
        let response = self.perform_geocoding(request).await?;
        
        // Cache the result
        self.cache.insert(cache_key, response.clone());
        
        info!("Successfully geocoded address: {} -> ({}, {})", 
              request.address, response.coordinates.latitude, response.coordinates.longitude);
        
        Ok(response)
    }

    async fn perform_geocoding(&self, request: &GeocodingRequest) -> Result<GeocodingResponse, ServiceError> {
        let url = self.build_geocoding_url(request);
        
        let response = timeout(
            Duration::from_secs(10),
            self.client.get(&url).send()
        ).await
        .map_err(|_| ServiceError::Timeout("Geocoding request timed out".to_string()))?
        .map_err(|e| ServiceError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            return Err(ServiceError::ApiError(
                format!("Geocoding API returned status: {}", response.status())
            ));
        }

        let osm_results: Vec<OpenStreetMapResponse> = response
            .json()
            .await
            .map_err(|