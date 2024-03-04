use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use super::games::Generation;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Ability {
    pub id: i32,
    pub name: String,
    pub is_main_series: bool,
    pub generation: Generation,
    pub effect_entry: String,
    pub effect_changes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
