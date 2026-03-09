use axum::{
    routing::{get, patch, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod cache;
mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("vr_www_api=debug".parse()?),
        )
        .init();

    // Load config
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    // Create database pool
    let pool = db::create_pool(&config.database_url).await?;

    // Run migrations
    db::run_migrations(&pool).await?;

    // Initialize state
    let state = AppState::new(config.clone(), pool).await?;

    // Build router with nested structure
    let signatures_routes = Router::new()
        .route("/", get(routes::list_signatures).post(routes::create_signature))
        .route("/{id}", get(routes::get_signature).put(routes::update_signature));

    let tracking_routes = Router::new()
        .route("/sessions", post(routes::create_session))
        .route("/events", post(routes::create_event))
        .route("/pageviews", post(routes::create_page_view))
        .route("/pageviews/{id}", patch(routes::update_page_view))
        .route("/stats", get(routes::get_stats));

    let api_routes = Router::new()
        .nest("/signatures", signatures_routes)
        .nest("/tracking", tracking_routes)
        .route("/geo/buildings/stream", get(routes::stream_geo_buildings))
        .route("/geo/buildings/{key}", get(routes::get_cached_buildings))
        .route("/contact", post(routes::submit_contact));

    // Start background GML cache eviction task
    tokio::spawn(routes::gml_cache_eviction(
        state.db.clone(),
        config.gml_cache_max_bytes,
    ));

    let app = Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
