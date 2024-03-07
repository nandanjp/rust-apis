use serde::{Deserialize, Serialize};

use crate::{
    models::{
        enums::{games::Game, region::Region, stats::Stat, types::Type},
        pokemon::{Pokemon, PokemonSerial},
    },
    utils::traits::GeneralAdaptor,
};

#[derive(Debug, Serialize)]
struct PokemonResponse {
    success: bool,
    pokemon: Option<PokemonSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListPokemonResponse {
    success: bool,
    pokemon: Option<Vec<PokemonSerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePokemon {
    name: String,
    base_experience: i32,
    height: f32,
    weight: f32,
    primary_type: Type,
    secondary_type: Option<Type>,
    primary_ability: String,
    secondary_ability: Option<String>,
    hidden_ability: Option<String>,
    is_main_series: Option<bool>,
    pokedex: i32,
    origin_region: Region,
    games: Vec<Game>,
    form_names: Vec<String>,
    is_mythical: bool,
    is_legendary: bool,
    stats: Vec<CreateStats>,
    sprites: CreateSprites,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateStats {
    kind: Stat,
    value: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSprites {
    front_default: String,
    front_shiny: String,
    front_female: String,
    front_shiny_female: String,
    back_default: String,
    back_shiny: String,
    back_female: String,
    back_shiny_female: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryPokemon {}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePokemon {
    name: String,
}

#[derive(Debug, Clone)]
struct PokemonError(String);
impl std::fmt::Display for PokemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve pokemon service due to the following error: {:#?}",
            self.0
        )
    }
}

struct PokemonService;
impl GeneralAdaptor for PokemonService {
    type Error = PokemonError;
    type Query = QueryPokemon;
    type UpdateBody = UpdatePokemon;
    type CreateBody = CreatePokemon;
    type Response = Pokemon;
    type ListResponse = Vec<Pokemon>;

    async fn get_all_items(
        pool: &sqlx::PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        todo!()
    }

    async fn get_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn create_one_item(
        pool: &sqlx::PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn update_one_item(
        id: i32,
        pool: &sqlx::PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn delete_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        todo!()
    }
}
