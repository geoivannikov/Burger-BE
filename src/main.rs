mod handlers;
mod models;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/get-burgers", get(handlers::get_burgers))
        .route("/health", get(handlers::health_check))
        .route("/images/:filename", get(handlers::get_burger_image))
        .nest_service("/", ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Server starting on http://{}", addr);
    println!("📋 Available endpoints:");
    println!("   GET  /get-burgers");
    println!("   GET  /images/:filename");
    println!("   GET  /health");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
