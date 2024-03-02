use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerializable;

#[derive(FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub destination: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerializable<OrderSerializable> for Order {
    fn to_serial(self) -> OrderSerializable {
        OrderSerializable {
            order_id: self.id,
            user_id: self.user_id,
            destination: self.destination,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct OrderSerializable {
    pub order_id: i32,
    pub user_id: i32,
    pub destination: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateOrder {
    pub customer: String,
    pub products: Vec<String>,
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateOrder {
    pub products: Option<Vec<String>>,
    pub destination: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct OrderResponse {
    pub success: bool,
    pub order: Option<OrderSerializable>,
    pub error: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ListOrderResponse {
    pub success: bool,
    pub orders: Option<Vec<OrderSerializable>>,
    pub error: Option<String>,
}
