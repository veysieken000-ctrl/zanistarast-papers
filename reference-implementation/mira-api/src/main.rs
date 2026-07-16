use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Debug, Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "zanistarast-mira-api",
        status: "ok",
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Mira API listener should bind");

    tracing::info!("Mira API listening on http://{address}");

    axum::serve(listener, app)
        .await
        .expect("Mira API server should run");
}






