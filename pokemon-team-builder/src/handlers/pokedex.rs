use serde::{Deserialize, Serialize};

use crate::{
    models::{
        enums::{games::Game, region::Region, types::Type},
        games::{Pokedex, PokedexSerial},
        pokemon::{Pokemon, PokemonSerial},
    },
    utils::traits::{GeneralAdaptor, IntoSerial},
};

#[derive(Debug, Clone, Serialize)]
struct ListPokedexResponse {
    success: bool,
    pokedexes: Option<Vec<PokedexSerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PokedexResponse {
    success: bool,
    pokedex: Option<PokedexSerial>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryPokedex {
    region: Region,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePokedex {
    name: String,
    is_main_series: Option<bool>,
    description: String,
    region: Region,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePokedex {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct PokedexError(String);
impl std::fmt::Display for PokedexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve pokedex resource due to the following error: {:#?}",
            self.0
        )
    }
}

struct PokedexService;
impl GeneralAdaptor for PokedexService {
    type Error = PokedexError;
    type CreateBody = CreatePokedex;
    type UpdateBody = UpdatePokedex;
    type Response = Pokedex;
    type ListResponse = Vec<Pokedex>;
    type Query = QueryPokedex;

    async fn get_all_items(
        pool: &sqlx::PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        sqlx::query!(r#"select id, name, is_main_series, description, region as "region: Region", created_at, updated_at from pokedex where region = $1"#, query.region as Region).fetch_all(pool).await.map(|p| p.into_iter().map(|p| Pokedex {
            id: p.id,
            name: p.name,
            is_main_series: p.is_main_series.unwrap_or_default(),
            description: p.description,
            region: p.region,
            created_at: p.created_at.unwrap(),
            updated_at: p.updated_at.unwrap()
        }).collect::<Vec<Pokedex>>()).map_err(|e| PokedexError(format!("failed to retrieve all pokedexes due to the following error: {e:#?}")))
    }

    async fn get_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"select id, name, is_main_series, description, region as "region: Region", created_at, updated_at from pokedex where id = $1"#, id).fetch_one(pool).await.map(|p| Pokedex {
            id: p.id,
            name: p.name,
            is_main_series: p.is_main_series.unwrap_or_default(),
            description: p.description,
            region: p.region,
            created_at: p.created_at.unwrap(),
            updated_at: p.updated_at.unwrap()
        }).map_err(|e| PokedexError(format!("failed to retrieve a pokedex with id = {id} due to the following error: {e:#?}")))
    }

    async fn create_one_item(
        pool: &sqlx::PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"insert into pokedex (name, is_main_series, description, region) values ($1, $2, $3, $4) returning id, name, is_main_series, description, region as "region: Region", created_at, updated_at"#, create.name, create.is_main_series.unwrap_or_default(), create.description, create.region as Region).fetch_one(pool).await.map(|p| Pokedex {
            id: p.id,
            name: p.name,
            is_main_series: p.is_main_series.unwrap_or_default(),
            description: p.description,
            region: p.region,
            created_at: p.created_at.unwrap(),
            updated_at: p.updated_at.unwrap()
        }).map_err(|e| PokedexError(format!("failed to create a pokedex from the given details all pokedexes due to the following error: {e:#?}")))
    }

    async fn update_one_item(
        id: i32,
        pool: &sqlx::PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        match update {
            UpdatePokedex{description: Some(des), name: Some(name)} => {
                sqlx::query!(r#"update pokedex set name = $1, description = $2 where id = $3 returning id, name, is_main_series, description, region as "region: Region", created_at, updated_at"#, name, des, id).fetch_one(pool).await.map(|p| Pokedex {
                    id: p.id,
                    name: p.name,
                    is_main_series: p.is_main_series.unwrap_or_default(),
                    description: p.description,
                    region: p.region,
                    created_at: p.created_at.unwrap(),
                    updated_at: p.updated_at.unwrap()
                }).map_err(|e| PokedexError(format!("failed to update the pokedex with id = {id} and with the given details all pokedexes due to the following error: {e:#?}")))
            },
            UpdatePokedex{name: Some(name),..} => {
                sqlx::query!(r#"update pokedex set name = $1 where id = $2 returning id, name, is_main_series, description, region as "region: Region", created_at, updated_at"#, name, id).fetch_one(pool).await.map(|p| Pokedex {
                    id: p.id,
                    name: p.name,
                    is_main_series: p.is_main_series.unwrap_or_default(),
                    description: p.description,
                    region: p.region,
                    created_at: p.created_at.unwrap(),
                    updated_at: p.updated_at.unwrap()
                }).map_err(|e| PokedexError(format!("failed to update the pokedex with id = {id} and with the given details all pokedexes due to the following error: {e:#?}")))
            },
            UpdatePokedex{description: Some(des), ..} => {
                sqlx::query!(r#"update pokedex set description = $1 where id = $2 returning id, name, is_main_series, description, region as "region: Region", created_at, updated_at"#, des, id).fetch_one(pool).await.map(|p| Pokedex {
                    id: p.id,
                    name: p.name,
                    is_main_series: p.is_main_series.unwrap_or_default(),
                    description: p.description,
                    region: p.region,
                    created_at: p.created_at.unwrap(),
                    updated_at: p.updated_at.unwrap()
                }).map_err(|e| PokedexError(format!("failed to update the pokedex with id = {id} and with the given details all pokedexes due to the following error: {e:#?}")))
            },
            _ => {
                sqlx::query!(r#"select id, name, is_main_series, description, region as "region: Region", created_at, updated_at from pokedex where id = $1"#, id).fetch_one(pool).await.map(|p| Pokedex {
                    id: p.id,
                    name: p.name,
                    is_main_series: p.is_main_series.unwrap_or_default(),
                    description: p.description,
                    region: p.region,
                    created_at: p.created_at.unwrap(),
                    updated_at: p.updated_at.unwrap()
                }).map_err(|e| PokedexError(format!("failed to update the pokedex with id = {id} and with the given details all pokedexes due to the following error: {e:#?}")))
            }
        }
    }

    async fn delete_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"delete from pokedex where id = $1 returning id, name, is_main_series, description, region as "region: Region", created_at, updated_at"#, id).fetch_one(pool).await.map(|p| Pokedex {
            id: p.id,
            name: p.name,
            is_main_series: p.is_main_series.unwrap_or_default(),
            description: p.description,
            region: p.region,
            created_at: p.created_at.unwrap(),
            updated_at: p.updated_at.unwrap()
        }).map_err(|e| PokedexError(format!("failed to delete the pokedex with id = {id} due to the following error: {e:#?}")))
    }
}

impl PokedexService {
    async fn get_all_pokemon(id: i32, pool: &sqlx::PgPool) -> Result<Vec<Pokemon>, PokedexError> {
        let pokemon = sqlx::query!(r#"select id, name, base_experience, height, weight, primary_type as "primary_type: Type", secondary_type as "secondary_type: Type", primary_ability, secondary_ability, hidden_ability, is_main_series, pokedex, origin_region as "origin_region: Region", games as "games: Vec<Game>", form_names, is_mythical, is_legendary, created_at, updated_at from pokemon where pokedex = $1"#, id).fetch_all(pool).await.map_err(|e| PokedexError(format!("failed to retrieve all pokemon in the pokedex with id = {id} due to the following error: {e:#?}")))?;

        let mut converted = Vec::new();
        for p in pokemon.into_iter() {
            let primary_ability =
                sqlx::query!("select name from ability where id = $1", p.primary_ability)
                    .fetch_one(pool)
                    .await.map_err(|e| PokedexError(format!("could not find an ability with an id = {} for the pokemon = {}: error = {e:#?}", p.primary_ability, p.name.clone())))?.name;
            let secondary_ability = match p.secondary_ability {
                Some(id) => match sqlx::query!("select name from ability where id = $1", id)
                    .fetch_one(pool)
                    .await
                {
                    Ok(ability) => Some(ability.name),
                    Err(_) => None,
                },
                None => None,
            };
            let hidden_ability = match p.hidden_ability {
                Some(id) => match sqlx::query!("select name from ability where id = $1", id)
                    .fetch_one(pool)
                    .await
                {
                    Ok(ability) => Some(ability.name),
                    Err(_) => None,
                },
                None => None,
            };
            converted.push(Pokemon {
                id: p.id,
                base_experience: p.base_experience,
                created_at: p.created_at.unwrap(),
                form_names: p.form_names.unwrap_or_default(),
                games: p.games,
                height: p.height,
                hidden_ability,
                secondary_ability,
                primary_type: p.primary_type,
                secondary_type: p.secondary_type,
                is_main_series: p.is_main_series.unwrap_or_default(),
                is_mythical: p.is_mythical.unwrap_or(false),
                is_legendary: p.is_legendary.unwrap_or(false),
                name: p.name,
                origin_region: p.origin_region,
                pokedex: p.pokedex,
                primary_ability,
                updated_at: p.updated_at.unwrap(),
                weight: p.weight,
            })
        }
        Ok(converted)
    }
}

use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_pokedexes(
    State(pool): State<PgPool>,
    Query(query): Query<QueryPokedex>,
) -> impl IntoResponse {
    match PokedexService::get_all_items(&pool, query).await {
        Ok(pokedexes) => (
            StatusCode::OK,
            Json(ListPokedexResponse {
                success: true,
                error: None,
                pokedexes: Some(
                    pokedexes
                        .into_iter()
                        .map(|p| p.to_serial())
                        .collect::<Vec<PokedexSerial>>(),
                ),
            }),
        ),
        Err(err) => (
            StatusCode::OK,
            Json(ListPokedexResponse {
                success: false,
                error: Some(format!(
                    "failed to retrieve all pokedexes due to the error = {err:#?}"
                )),
                pokedexes: None,
            }),
        ),
    }
}

pub async fn get_pokedex_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match PokedexService::get_one_item(id, &pool).await {
        Ok(pokedex) => (StatusCode::OK, Json(PokedexResponse {
            success: true,
            pokedex: Some(pokedex.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(PokedexResponse {
            success: false,
            pokedex: None,
            error: Some(format!("failed to retrieve a pokedex with the id = {id} due to the following error: {err:#?}"))
        }))
    }
}

pub async fn create_pokedex(
    State(pool): State<PgPool>,
    Json(create): Json<CreatePokedex>,
) -> impl IntoResponse {
    match PokedexService::create_one_item(&pool, create).await {
        Ok(pokedex) => (StatusCode::CREATED, Json(PokedexResponse {
            success: true,
            pokedex: Some(pokedex.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(PokedexResponse {
            success: false,
            pokedex: None,
            error: Some(format!("failed to create a pokedex with the provided details due to the following error: {err:#?}"))
        }))
    }
}

pub async fn update_pokedex(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdatePokedex>,
) -> impl IntoResponse {
    match PokedexService::update_one_item(id, &pool, update).await {
        Ok(pokedex) => (StatusCode::OK, Json(PokedexResponse {
            success: true,
            pokedex: Some(pokedex.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(PokedexResponse {
            success: false,
            pokedex: None,
            error: Some(format!("failed to update the pokedex with id = {id} with the provided details due to the following error: {err:#?}"))
        }))
    }
}

pub async fn delete_pokedex(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match PokedexService::delete_one_item(id, &pool).await {
        Ok(pokedex) => (StatusCode::OK, Json(PokedexResponse {
            success: true,
            pokedex: Some(pokedex.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(PokedexResponse {
            success: false,
            pokedex: None,
            error: Some(format!("failed to delete the pokedex with id = {id} due to the following error: {err:#?}"))
        }))
    }
}

#[derive(Debug, Serialize)]
struct ListPokemonResponse {
    success: bool,
    pokemon: Option<Vec<PokemonSerial>>,
    error: Option<String>,
}
pub async fn get_pokemon_from_pokedex(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match PokedexService::get_all_pokemon(id, &pool).await {
        Ok(pokemon) => (StatusCode::OK, Json(ListPokemonResponse {
            success: true,
            pokemon: Some(pokemon.into_iter().map(|p| p.to_serial()).collect::<Vec<PokemonSerial>>()),
            error: None
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(ListPokemonResponse {
            success: false,
            pokemon: None,
            error: Some(format!("failed to get all pokemon in the pokedex with id = {id} due to the following error: {err:#?}"))
        }))
    }
}
