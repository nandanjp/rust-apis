use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

use super::enums::tier::Tier;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub tier: Tier,
    pub is_favourite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for Team {
    type Serial = TeamSerial;

    fn to_serial(&self) -> Self::Serial {
        TeamSerial {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            user_id: self.user_id,
            tier: self.tier.clone(),
            is_favourite: self.is_favourite,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TeamSerial {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub tier: Tier,
    pub is_favourite: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTeam {
    pub name: String,
    pub description: String,
    pub user: i32,
    pub tier: Tier,
    pub is_favourite: Option<bool>,
}
