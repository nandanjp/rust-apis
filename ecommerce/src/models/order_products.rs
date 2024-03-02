use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct OrderProducts {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
