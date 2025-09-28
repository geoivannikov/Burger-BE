mod models;
mod handlers;

use axum::{
    routing::get,
    Router,
};
use tower_http::{
    cors::{CorsLayer, Any},
    services::ServeDir,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/get-burgers", get(handlers::get_burgers))
        .route("/health", get(handlers::health_check))
        .nest_service("/", ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("🚀 Server starting on http://{}", addr);
    println!("📋 Available endpoints:");
    println!("   GET  /get-burgers");

    // Запускаем сервер
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}