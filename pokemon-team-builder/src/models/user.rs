use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
