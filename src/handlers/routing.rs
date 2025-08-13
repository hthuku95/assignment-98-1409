use crate::models::{Coordinate, Route, RouteRequest, RouteResponse, NavigationStep, TravelMode};
use crate::services::map_service::MapService;
use crate::utils::distance::calculate_distance;
use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, error};

pub struct RoutingHandler {
    map_service: Arc<RwLock<MapService>>,
    route_cache: Arc<RwLock<HashMap<String, Route>>>,
}

impl RoutingHandler {
    pub fn new(map_service: Arc<RwLock<MapService>>) -> Self {
        Self {
            map_service,
            route_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn calculate_route(&self, req: web::Json<RouteRequest>) -> Result<HttpResponse> {
        info!("Calculating route from {:?} to {:?}", req.origin, req.destination);

        // Validate request
        if let Err(validation_error) = self.validate_route_request(&req).await {
            warn!("Route request validation failed: {}", validation_error);
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "Invalid route request",
                "message": validation_error
            })));
        }

        // Check cache first
        let cache_key = self.generate_cache_key(&req);
        if let Some(cached_route) = self.get_cached_route(&cache_key).await {
            info!("Returning cached route for key: {}", cache_key);
            return Ok(HttpResponse::Ok().json(RouteResponse {
                route: cached_route,
                status: "OK".to_string(),
                request_id: uuid::Uuid::new_v4().to_string(),
            }));
        }

        // Calculate new route
        match self.compute_optimal_route(&req).await {
            Ok(route) => {
                // Cache the result
                self.cache_route(cache_key, route.clone()).await;
                
                info!("Route calculated successfully: {} km, {} minutes", 
                      route.distance_km, route.duration_minutes);

                Ok(HttpResponse::Ok().json(RouteResponse {
                    route,
                    status: "OK".to_string(),
                    request_id: uuid::Uuid::new_v4().to_string(),
                }))
            }
            Err(e) => {
                error!("Failed to calculate route: {}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Route calculation failed",
                    "message": e
                })))
            }
        }
    }

    pub async fn get_directions(&self, req: web::Json<RouteRequest>) -> Result<HttpResponse> {
        info!("Getting turn-by-turn directions");

        match self.calculate_route(req).await {
            Ok(response) => {
                let body = response.into_body();
                // Parse the response to extract route and generate detailed directions
                Ok(HttpResponse::Ok().json(json!({
                    "directions": self.generate_turn_by_turn_directions().await,
                    "status": "OK"
                })))
            }
            Err(e) => {
                error!("Failed to get directions: {:?}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Directions unavailable"
                })))
            }
        }
    }

    pub async fn get_alternative_routes(&self, req: web::Json<RouteRequest>) -> Result<HttpResponse> {
        info!("Calculating alternative routes");

        let mut routes = Vec::new();
        
        // Calculate primary route
        if let Ok(primary_route) = self.compute_optimal_route(&req).await {
            routes.push(primary_route);
        }

        // Calculate alternative routes with different parameters