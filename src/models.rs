use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Burger {
    pub id: u32,
    pub title: String,
    pub price: String,
    pub img: String,
}

impl Burger {
    pub fn new(id: u32, title: String, price: String, img: String) -> Self {
        Self {
            id,
            title,
            price,
            img,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct NewsSubscribe {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}