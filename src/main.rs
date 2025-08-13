use actix_web::{web, App, HttpServer, Result, HttpResponse, middleware::Logger};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use env_logger;

mod routes;
mod models;
mod services;

use routes::{map_routes, search_routes, directions_routes};
use models::{Location, SearchResult, Route};
use services::{MapService, SearchService, DirectionsService};

#[derive(Clone)]
pub struct AppState {
    pub map_service: MapService,
    pub search_service: SearchService,
    pub directions_service: DirectionsService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            map_service: MapService::new(),
            search_service: SearchService::new(),
            directions_service: DirectionsService::new(),
        }
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "maps-clone",
        "version": "1.0.0"
    })))
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Maps Clone</title>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="/static/css/main.css">
        </head>
        <body>
            <div id="app">
                <div id="search-container">
                    <input type="text" id="search-input" placeholder="Search for places...">
                    <button id="search-btn">Search</button>
                    <button id="directions-btn">Directions</button>
                </div>
                <div id="map-container">
                    <div id="map"></div>
                </div>
                <div id="sidebar">
                    <div id="search-results"></div>
                    <div id="directions-panel"></div>
                </div>
            </div>
            <script src="/static/js/map.js"></script>
            <script src="/static/js/search.js"></script>
            <script src="/static/js/directions.js"></script>
            <script src="/static/js/main.js"></script>
        </body>
        </html>
        "#
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState::new());
    
    log::info!("Starting Maps Clone server on http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .configure(map_routes::configure)
                    .configure(search_routes::configure)
                    .configure(directions_routes::configure)
            )
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}