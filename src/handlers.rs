use axum::{response::Json, http::StatusCode};
use serde_json::json;
use crate::models::Burger;

pub async fn get_burgers() -> Result<Json<serde_json::Value>, StatusCode> {
    let burgers = vec![
        Burger::new(1, "Classic Beef Supreme".to_string(), "11.50".to_string(), "/B1.png".to_string()),
        Burger::new(2, "Smoky Bacon Tower".to_string(), "12.20".to_string(), "/B2.png".to_string()),
        Burger::new(3, "Texas BBQ Beef".to_string(), "12.80".to_string(), "/B3.png".to_string()),
        Burger::new(4, "Double Meat Monster".to_string(), "13.50".to_string(), "/B4.png".to_string()),
        Burger::new(5, "Hot Chili Beef Blast".to_string(), "12.00".to_string(), "/b4.png".to_string()),
        Burger::new(6, "Veg Chizy Burger".to_string(), "9.50".to_string(), "/b4.png".to_string()),
        Burger::new(7, "Triple Cheese Melt".to_string(), "11.20".to_string(), "/b4.png".to_string()),
        Burger::new(8, "Cheddar Crunch Burger".to_string(), "10.80".to_string(), "/b4.png".to_string()),
        Burger::new(9, "Blue Cheese Deluxe".to_string(), "12.40".to_string(), "/b4.png".to_string()),
        Burger::new(10, "Mozzarella Magic".to_string(), "11.00".to_string(), "/b4.png".to_string()),
        Burger::new(11, "Swiss Alps Cheese Burger".to_string(), "12.90".to_string(), "/b4.png".to_string()),
        Burger::new(12, "Green Delight Burger".to_string(), "8.90".to_string(), "/b4.png".to_string()),
        Burger::new(13, "Spicy Bean Crunch".to_string(), "9.20".to_string(), "/b4.png".to_string()),
        Burger::new(14, "Avocado Dream Burger".to_string(), "9.80".to_string(), "/b4.png".to_string()),
        Burger::new(15, "Vegan BBQ Smoke".to_string(), "10.50".to_string(), "/b4.png".to_string()),
        Burger::new(16, "Mediterranean Veggie Bite".to_string(), "9.90".to_string(), "/b4.png".to_string()),
    ];

    Ok(Json(json!({
        "burgers": burgers
    })))
}

pub async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "message": "Server is running"
    })))
}