use crate::{models::enums::types::Type, utils::traits::IntoSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

impl IntoSerial for Move {
    type Serial = MoveSerial;

    fn to_serial(&self) -> Self::Serial {
        MoveSerial {
            id: self.id,
            name: self.name.clone(),
            accuracy: self.accuracy,
            effect_change: self.effect_change,
            pp: self.pp,
            priority: self.priority,
            power: self.power,
            type_: self.type_.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MoveSerial {
    pub id: i32,
    pub name: String,
    pub accuracy: Option<i32>,
    pub effect_change: Option<i32>,
    pub pp: Option<i32>,
    pub priority: i32,
    pub power: Option<i32>,
    #[serde(rename = "type")]
    pub type_: Type,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateMove {
    pub name: String,
    pub accuracy: Option<i32>,
    pub effect_change: Option<i32>,
    pub pp: Option<i32>,
    pub priority: i32,
    pub power: Option<i32>,
    #[serde(rename = "type")]
    pub type_: Type,
}
