use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct TeamPokemon {
    pub id: i32,
    pub team_id: i32,
    pub pokemon: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for TeamPokemon {
    type Serial = TeamPokemonSerial;

    fn to_serial(&self) -> Self::Serial {
        TeamPokemonSerial {
            id: self.id,
            team_id: self.team_id,
            pokemon: self.pokemon.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TeamPokemonSerial {
    pub id: i32,
    pub team_id: i32,
    pub pokemon: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTeamPokemon {
    pub team_id: i32,
    pub pokemon: String,
}
