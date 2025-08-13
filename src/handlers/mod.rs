pub mod auth;
pub mod maps;
pub mod routes;
pub mod search;
pub mod directions;
pub mod places;
pub mod geocoding;

use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "googlemaps-clone",
        "version": "1.0.0"
    })))
}

pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(json!({
        "error": "Not Found",
        "message": "The requested resource was not found"
    })))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .configure(auth::configure_auth_routes)
            )
            .service(
                web::scope("/maps")
                    .configure(maps::configure_maps_routes)
            )
            .service(
                web::scope("/search")
                    .configure(search::configure_search_routes)
            )
            .service(
                web::scope("/directions")
                    .configure(directions::configure_directions_routes)
            )
            .service(
                web::scope("/places")
                    .configure(places::configure_places_routes)
            )
            .service(
                web::scope("/geocoding")
                    .configure(geocoding::configure_geocoding_routes)
            )
            .route("/health", web::get().to(health_check))
    )
    .default_service(web::route().to(not_found));
}