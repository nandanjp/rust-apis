use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct MovePokemon {
    id: i32,
    #[sqlx(rename = "move")]
    move_: String,
    pokemon: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IntoSerial for MovePokemon {
    type Serial = MovePokemonSerial;

    fn to_serial(&self) -> Self::Serial {
        MovePokemonSerial {
            move_: self.move_.clone(),
            pokemon: self.pokemon.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MovePokemonSerial {
    #[serde(rename = "move")]
    move_: String,
    pokemon: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddMoveToPokemon {
    #[serde(rename = "move")]
    move_: String,
    pokemon: String,
}
