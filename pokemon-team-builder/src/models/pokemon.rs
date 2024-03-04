use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use super::enums::{games::Game, region::Region, types::Type};

#[derive(Debug, PartialEq, PartialOrd, FromRow)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub base_experience: Option<i32>,
    pub height: i32,
    pub weight: i32,
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
    pub stats: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, FromRow)]
pub struct PokemonSprites {
    pub id: i32,
    pub pokedex: i32,
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
