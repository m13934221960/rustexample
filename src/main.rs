mod model;

use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use chrono::Month::January;

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE : &str = "Build Simple CRUD API in Rust using Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });
    Json(json_response)
}
# [tokio::main]
async fn main() {
    let app = Router::new().route("/api/health-checker",get(health_check_handler));
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
