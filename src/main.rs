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
        .nest_service("/", ServeDir::new("static"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    println!("🚀 Server starting on http://{}", addr);
    println!("📋 Available endpoints:");
    println!("   GET  /get-burgers");

    // Запускаем сервер
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}