use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use super::enums::{region::Region, types::Type};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Generation {
    pub id: i32,
    pub name: String,
    pub main_region: Region,
    pub types: Vec<Type>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Pokedex {
    pub id: i32,
    pub name: String,
    pub is_main_series: bool,
    pub description: String,
    pub region: Region,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
