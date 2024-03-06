use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

use super::enums::{games::Game, region::Region, types::Type};

#[derive(Debug, PartialEq, PartialOrd, FromRow)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub base_experience: Option<i32>,
    pub height: i32,
    pub weight: i32,
    pub primary_type: Type,
    pub secondary_type: Option<Type>,
    pub primary_ability: String,
    pub secondary_ability: Option<String>,
    pub hidden_ability: Option<String>,
    pub is_main_series: bool,
    pub pokedex: i32,
    pub origin_region: Region,
    pub games: Vec<Game>,
    pub form_names: Vec<String>,
    pub is_mythical: bool,
    pub is_legendary: bool,
    pub types: Vec<Type>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for Pokemon {
    type Serial = PokemonSerial;
    fn to_serial(&self) -> Self::Serial {
        PokemonSerial {
            id: self.id,
            name: self.name.clone(),
            base_experience: self.base_experience,
            height: self.height,
            weight: self.weight,
            primary_type: self.primary_type.clone(),
            secondary_type: self.secondary_type.clone(),
            primary_ability: self.primary_ability.clone(),
            secondary_ability: self.secondary_ability.clone(),
            hidden_ability: self.hidden_ability.clone(),
            is_main_series: self.is_main_series,
            pokedex: self.pokedex,
            origin_region: self.origin_region.clone(),
            games: self.games.clone(),
            form_names: self.form_names.clone(),
            is_mythical: self.is_mythical,
            is_legendary: self.is_legendary,
            types: self.types.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonSerial {
    pub id: i32,
    pub name: String,
    pub base_experience: Option<i32>,
    pub height: i32,
    pub weight: i32,
    pub primary_type: Type,
    pub secondary_type: Option<Type>,
    pub primary_ability: String,
    pub secondary_ability: Option<String>,
    pub hidden_ability: Option<String>,
    pub is_main_series: bool,
    pub pokedex: i32,
    pub origin_region: Region,
    pub games: Vec<Game>,
    pub form_names: Vec<String>,
    pub is_mythical: bool,
    pub is_legendary: bool,
    pub types: Vec<Type>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreatePokemon {
    pub name: String,
    pub base_experience: Option<i32>,
    pub height: i32,
    pub weight: i32,
    pub primary_type: Type,
    pub secondary_type: Option<Type>,
    pub primary_ability: String,
    pub secondary_ability: Option<String>,
    pub hidden_ability: Option<String>,
    pub is_main_series: Option<bool>,
    pub pokedex: i32,
    pub origin_region: Region,
    pub games: Vec<Game>,
    pub form_names: Option<Vec<String>>,
    pub is_mythical: Option<bool>,
    pub is_legendary: Option<bool>,
    pub types: Vec<Type>,
    pub stats: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct PokemonSprites {
    pub id: i32,
    pub pokemon: String,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for PokemonSprites {
    type Serial = PokemonSpritesSerial;

    fn to_serial(&self) -> Self::Serial {
        PokemonSpritesSerial {
            id: self.id,
            pokemon: self.pokemon.clone(),
            front_default: self.front_default.clone(),
            front_shiny: self.front_shiny.clone(),
            front_female: self.front_female.clone(),
            front_shiny_female: self.front_shiny_female.clone(),
            back_default: self.back_default.clone(),
            back_shiny: self.back_shiny.clone(),
            back_female: self.back_female.clone(),
            back_shiny_female: self.back_shiny_female.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonSpritesSerial {
    pub id: i32,
    pub pokemon: String,
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
