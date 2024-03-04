use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

use super::enums::stats::Stat;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct PokemonStat {
    pub id: i32,
    pub pokemon: String,
    pub kind: Stat,
    pub value: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for PokemonStat {
    type Serial = PokemonStatSerial;

    fn to_serial(&self) -> Self::Serial {
        PokemonStatSerial {
            id: self.id,
            pokemon: self.pokemon.clone(),
            kind: self.kind.clone(),
            value: self.value,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PokemonStatSerial {
    pub id: i32,
    pub pokemon: String,
    pub kind: Stat,
    pub value: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePokemonStat {
    pub pokemon: String,
    pub kind: Stat,
    pub value: i32,
}
