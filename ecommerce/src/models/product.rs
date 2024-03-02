use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::{IntoSerializable, SerializeEnum};

use super::enums::category::Category;

#[derive(FromRow)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub price: f32,
    pub quantity_available: i32,
    pub category: Category,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerializable<ProductSerializable> for Product {
    fn to_serial(self) -> ProductSerializable {
        ProductSerializable {
            product_id: self.id,
            title: self.title,
            description: self.description,
            price: self.price,
            quantity_available: self.quantity_available,
            category: self.category.to_string().to_string(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ProductSerializable {
    pub product_id: i32,
    pub title: String,
    pub description: String,
    pub price: f32,
    pub quantity_available: i32,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateProduct {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub quantity_available: u64,
    pub category: Category,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateProduct {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub quantity_available: Option<f64>,
    pub category: Option<Category>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProductResponse {
    pub success: bool,
    pub product: Option<ProductSerializable>,
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ListProductResponse {
    pub success: bool,
    pub products: Option<Vec<ProductSerializable>>,
    pub error: Option<String>,
}

impl CreateProduct {}
impl UpdateProduct {}
