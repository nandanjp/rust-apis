use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

use super::enums::games::Game;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Ability {
    pub id: i32,
    pub name: String,
    pub is_main_series: bool,
    pub generation: Game,
    pub effect_entry: String,
    pub effect_changes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for Ability {
    type Serial = AbilitySerial;

    fn to_serial(&self) -> Self::Serial {
        AbilitySerial {
            id: self.id,
            name: self.name.clone(),
            is_main_series: self.is_main_series,
            generation: self.generation.clone(),
            effect_entry: self.effect_entry.clone(),
            effect_changes: self.effect_changes.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AbilitySerial {
    pub id: i32,
    pub name: String,
    pub is_main_series: bool,
    pub generation: Game,
    pub effect_entry: String,
    pub effect_changes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateAbility {
    pub name: String,
    pub is_main_series: bool,
    pub generation: Game,
    pub effect_entry: String,
    pub effect_changes: String,
}
