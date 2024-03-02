use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerializable;

#[derive(FromRow)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerializable<CartSerializable> for Cart {
    fn to_serial(self) -> CartSerializable {
        CartSerializable {
            cart_id: self.id,
            user_id: self.user_id,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CartSerializable {
    pub cart_id: i32,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateCart {
    pub customer: String,
    pub email: String,
    pub products: Option<Vec<String>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct CartResponse {
    pub success: bool,
    pub cart: Option<CartSerializable>,
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ListCartResponse {
    pub success: bool,
    pub carts: Option<Vec<CartSerializable>>,
    pub error: Option<String>,
}
