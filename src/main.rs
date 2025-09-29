use axum::{
    http::{Method, StatusCode},
    response::{Html, Json},
    routing::get,
    Router,
};
use serde_json::json;
use std::{env, time::SystemTime};
use tokio::fs;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

// Store start time for uptime calculation
static START_TIME: std::sync::LazyLock<SystemTime> = std::sync::LazyLock::new(SystemTime::now);

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // Build the application with routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/health", get(health_check))
        .route("/ping", get(ping_pong))
        .layer(ServiceBuilder::new().layer(cors));

    println!("🦀 Rust Axum server starting on port {}", port);
    println!("📊 Health endpoint: http://localhost:{}/health", port);
    println!("🏓 Ping endpoint: http://localhost:{}/ping", port);
    println!("🌐 Frontend: http://localhost:{}/", port);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn serve_index() -> Result<Html<String>, StatusCode> {
    match fs::read_to_string("index.html").await {
        Ok(content) => Ok(Html(content)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn health_check() -> Json<serde_json::Value> {
    let uptime = START_TIME.elapsed().unwrap_or_default();
    let uptime_str = format!(
        "{}h{}m{}s",
        uptime.as_secs() / 3600,
        (uptime.as_secs() % 3600) / 60,
        uptime.as_secs() % 60
    );

    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime": uptime_str
    }))
}

async fn ping_pong() -> Json<serde_json::Value> {
    Json(json!({
        "message": "pong",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "server_id": "rust-axum-v1"
    }))
}
