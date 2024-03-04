use crate::models::enums::types::Type;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Move {
    pub id: i32,
    pub name: String,
    pub accuracy: Option<i32>,
    pub effect_change: Option<i32>,
    pub pp: Option<i32>,
    pub priority: i32,
    pub power: Option<i32>,
    #[sqlx(rename = "type")]
    pub type_: Type,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
