mod handlers;
mod models;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/get-burgers", get(handlers::get_burgers))
        .route("/images/:filename", get(handlers::get_burger_image))
        // .route("/news_subscribe", post(handlers::news_subscribe))
        .nest_service("/static", ServeDir::new("static"))
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
    println!("   GET  /");
    println!("   GET  /get-burgers");
    println!("   GET  /images/:filename");
    println!("   GET  /health");
    println!("   POST /news_subscribe");
    println!("🔧 Environment PORT: {:?}", std::env::var("PORT"));
    println!("🔧 Binding to: {}", addr);
    println!("🔧 Railway health check will use: http://0.0.0.0:{}/health", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("✅ Server successfully bound to {}", addr);
    println!("🌐 Health check available at: http://{}:{}/health", "0.0.0.0", port);
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to LingPing Backend API!"
}

async fn health_check() -> &'static str {
    "OK"
} 
