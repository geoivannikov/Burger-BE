use axum::{response::Json, http::StatusCode, response::Response, body::Body, extract::Path, extract::Json as ExtractJson};
use serde_json::json;
use std::path::Path as StdPath;
use tokio::fs;
use crate::models::{Burger, NewsSubscribe};

pub async fn get_burgers() -> Result<Json<serde_json::Value>, StatusCode> {
    let burgers = vec![
        Burger::new(1, "Classic Beef Supreme".to_string(), "11.50".to_string(), "/B1.png".to_string()),
        Burger::new(2, "Smoky Bacon Tower".to_string(), "12.20".to_string(), "/B2.png".to_string()),
        Burger::new(3, "Texas BBQ Beef".to_string(), "12.80".to_string(), "/B3.png".to_string()),
        Burger::new(4, "Double Meat Monster".to_string(), "13.50".to_string(), "/B4.png".to_string()),
        Burger::new(5, "Hot Chili Beef Blast".to_string(), "12.00".to_string(), "/B5.png".to_string()),
        Burger::new(6, "Veg Chizy Burger".to_string(), "9.50".to_string(), "/B6.png".to_string()),
        Burger::new(7, "Triple Cheese Melt".to_string(), "11.20".to_string(), "/B7.png".to_string()),
        Burger::new(8, "Cheddar Crunch Burger".to_string(), "10.80".to_string(), "/B8.png".to_string()),
        Burger::new(9, "Blue Cheese Deluxe".to_string(), "12.40".to_string(), "/B9.png".to_string()),
        Burger::new(10, "Swiss Alps Cheese Burger".to_string(), "12.90".to_string(), "/B10.png".to_string()),
        Burger::new(11, "Green Delight Burger".to_string(), "8.90".to_string(), "/B11.png".to_string()),
        Burger::new(12, "Spicy Bean Crunch".to_string(), "9.20".to_string(), "/B12.png".to_string()),
        Burger::new(13, "Avocado Dream Burger".to_string(), "9.80".to_string(), "/B13.png".to_string()),
        Burger::new(14, "Vegan BBQ Smoke".to_string(), "10.50".to_string(), "/B14.png".to_string()),
        Burger::new(15, "Mediterranean Veggie Bite".to_string(), "9.90".to_string(), "/B15.png".to_string()),
    ];

    Ok(Json(json!({
        "burgers": burgers
    })))
}

pub async fn get_burger_image(Path(filename): Path<String>) -> Result<Response<Body>, StatusCode> {
    let current_dir = std::env::current_dir().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_path = current_dir.join("static").join(&filename);
    
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    match fs::read(&file_path).await {
        Ok(contents) => {
            let mime_type = match StdPath::new(&file_path).extension().and_then(|s| s.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("webp") => "image/webp",
                _ => "application/octet-stream",
            };
            
            let response = Response::builder()
                .status(200)
                .header("Content-Type", mime_type)
                .header("Cache-Control", "public, max-age=3600") 
                .body(Body::from(contents))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(response)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "message": "Server is running"
    })))
}

pub async fn news_subscribe(ExtractJson(payload): ExtractJson<NewsSubscribe>) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("📧 News subscription request for email: {}", payload.email);
    
    
    if payload.email.is_empty() {
        return Ok(Json(json!({
            "status": "error",
            "message": "Email cannot be empty"
        })));
    }
    
    if payload.email.len() > 254 {
        return Ok(Json(json!({
            "status": "error",
            "message": "Email is too long"
        })));
    }
    
    println!("✅ Valid email received: {}", payload.email);
    
    Ok(Json(json!({
        "status": "success",
        "message": "Successfully subscribed to news",
        "email": payload.email
    })))
}