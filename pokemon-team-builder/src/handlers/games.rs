use serde::{Deserialize, Serialize};

use crate::{
    models::{
        enums::{region::Region, types::Type},
        games::{Generation, GenerationSerial},
    },
    utils::traits::{GeneralAdaptor, IntoSerial},
};

#[derive(Debug, Clone, Serialize)]
pub struct ListGenerationResponse {
    success: bool,
    generations: Option<Vec<GenerationSerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationResponse {
    success: bool,
    generation: Option<GenerationSerial>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GenerationError(String);
impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve the generation resource due to the following error: {0:#?}",
            self.0
        )
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GenerationQuery {
    name: Option<String>,
    main_region: Option<Region>,
    types: Option<Type>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateGeneration {
    name: String,
    main_region: Region,
    types: Vec<Type>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateGeneration {
    name: Option<String>,
    main_region: Option<Region>,
    types: Option<Vec<Type>>,
}

struct GenerationService;
impl GeneralAdaptor for GenerationService {
    type Error = GenerationError;
    type Query = GenerationQuery;
    type Response = Generation;
    type CreateBody = CreateGeneration;
    type UpdateBody = UpdateGeneration;
    type ListResponse = Vec<Generation>;

    async fn get_all_items(
        pool: &sqlx::PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query {
            GenerationQuery { main_region: Some(region), types: Some(typ), .. } => {
                sqlx::query!(r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation where main_region = $1 and $2 = any(types)"#, region as Region, typ as Type).fetch_all(pool).await.map(|g| g.into_iter().map(|g| Generation {
                    id: g.id,
                    name: g.name,
                    main_region: g.main_region,
                    types: g.types.unwrap(),
                    created_at: g.created_at.unwrap(),
                    updated_at: g.updated_at.unwrap(),
                }).collect::<Vec<Generation>>()).map_err(|e| GenerationError(format!("failed to retrieve all generations due to the following error: {e:#?}")))
            },
            GenerationQuery { name: Some(ref gen), .. } => {
                sqlx::query!(r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation where name = $1"#, *gen).fetch_all(pool).await.map(|g| g.into_iter().map(|g| Generation {
                    id: g.id,
                    name: g.name,
                    main_region: g.main_region,
                    types: g.types.unwrap(),
                    created_at: g.created_at.unwrap(),
                    updated_at: g.updated_at.unwrap(),
                }).collect::<Vec<Generation>>()).map_err(|e| GenerationError(format!("failed to retrieve all generations due to the following error: {e:#?}")))
            },
            GenerationQuery { main_region: Some(region), .. } => {
                sqlx::query!(r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation where main_region = $1"#, region as Region).fetch_all(pool).await.map(|g| g.into_iter().map(|g| Generation {
                    id: g.id,
                    name: g.name,
                    main_region: g.main_region,
                    types: g.types.unwrap(),
                    created_at: g.created_at.unwrap(),
                    updated_at: g.updated_at.unwrap(),
                }).collect::<Vec<Generation>>()).map_err(|e| GenerationError(format!("failed to retrieve all generations due to the following error: {e:#?}")))
            },
            GenerationQuery { types: Some(typ), .. } => {
                sqlx::query!(r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation where $1 = any(types)"#, typ as Type).fetch_all(pool).await.map(|g| g.into_iter().map(|g| Generation {
                    id: g.id,
                    name: g.name,
                    main_region: g.main_region,
                    types: g.types.unwrap(),
                    created_at: g.created_at.unwrap(),
                    updated_at: g.updated_at.unwrap(),
                }).collect::<Vec<Generation>>()).map_err(|e| GenerationError(format!("failed to retrieve all generations due to the following error: {e:#?}")))
            },
            _ => {
                sqlx::query!(r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation"#).fetch_all(pool).await.map(|g| g.into_iter().map(|g| Generation {
                    id: g.id,
                    name: g.name,
                    main_region: g.main_region,
                    types: g.types.unwrap(),
                    created_at: g.created_at.unwrap(),
                    updated_at: g.updated_at.unwrap(),
                }).collect::<Vec<Generation>>()).map_err(|e| GenerationError(format!("failed to retrieve all generations due to the following error: {e:#?}")))
            }
        }
    }

    async fn get_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation where id = $1"#, id).fetch_one(pool).await.map(|g| Generation {
            id: g.id,
            name: g.name,
            main_region: g.main_region,
            types: g.types.unwrap(),
            created_at: g.created_at.unwrap(),
            updated_at: g.updated_at.unwrap(),
        }).map_err(|e| GenerationError(format!("failed to retrieve a generation with the id = {id} due to the following error: {e:#?}")))
    }

    async fn create_one_item(
        pool: &sqlx::PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query!(
            r#"insert into generation (name, main_region, types) values ($1, $2, $3) returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
            create.name,
            create.main_region as Region,
            create.types as Vec<Type>,
        ).fetch_one(pool).await.map(|g| Generation {
            id: g.id,
            name: g.name,
            main_region: g.main_region,
            types: g.types.unwrap(),
            created_at: g.created_at.unwrap(),
            updated_at: g.updated_at.unwrap(),
        }).map_err(|e| GenerationError(format!("failed to create a new generation the given details due to the following error: {e:#?}")))
    }

    async fn update_one_item(
        id: i32,
        pool: &sqlx::PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        match update {
            UpdateGeneration { name: Some(name), main_region: Some(region), types: Some(types) } => {
                sqlx::query!(
                    r#"update generation set name = $1, main_region = $2, types = $3 where id = $4 returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
                    name,
                    region as Region,
                    types as Vec<Type>,
                    id
                ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to update the generation with id = {id} due to the following error: {e:#?}")))
            },
            UpdateGeneration { name: Some(name), main_region: Some(region), ..} => {
                sqlx::query!(
                    r#"update generation set name = $1, main_region = $2 where id = $3 returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
                    name,
                    region as Region,
                    id
                ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to update the generation with id = {id} due to the following error: {e:#?}")))
            },
            UpdateGeneration { name: Some(name), ..} => {
                sqlx::query!(
                    r#"update generation set name = $1 where id = $2 returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
                    name,
                    id
                ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to update the generation with id = {id} due to the following error: {e:#?}")))
            },
            UpdateGeneration { main_region: Some(region), types: Some(types), ..} => {
                sqlx::query!(
                    r#"update generation set main_region = $1, types = $2 where id = $3 returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
                    region as Region,
                    types as Vec<Type>,
                    id
                ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to update the generation with id = {id} due to the following error: {e:#?}")))
            },
            UpdateGeneration { types: Some(types), ..} => {
                sqlx::query!(
                    r#"update generation set types = $1 where id = $2 returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
                    types as Vec<Type>,
                    id
                ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to update the generation with id = {id} due to the following error: {e:#?}")))
            },
            _ => {
                sqlx::query!(
                    r#"select id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at from generation where id = $1"#,
                    id
                ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to update the generation with id = {id} due to the following error: {e:#?}")))
            }
        }
    }

    async fn delete_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(
            r#"delete from generation where id = $1 returning id, name, main_region as "main_region: Region", types as "types: Vec<Type>", created_at, updated_at"#,
            id
        ).fetch_one(pool).await.map(|g| Generation {id: g.id, name: g.name, main_region: g.main_region, types: g.types.unwrap(), created_at: g.created_at.unwrap(), updated_at: g.updated_at.unwrap()}).map_err(|e| GenerationError(format!("failed to delete the generation with id = {id} due to the following error: {e:#?}")))
    }
}

use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::postgres::PgPool;
pub async fn get_all_generations(
    State(pool): State<PgPool>,
    query: Option<Query<GenerationQuery>>,
) -> impl IntoResponse {
    match GenerationService::get_all_items(&pool, query.unwrap_or_default().0).await {
        Ok(generations) => (
            StatusCode::OK,
            Json(ListGenerationResponse {
                success: true,
                generations: Some(
                    generations
                        .into_iter()
                        .map(|g| g.to_serial())
                        .collect::<Vec<GenerationSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListGenerationResponse {
                success: false,
                generations: None,
                error: Some(format!(
                    "failed to retrieve all generations due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_generation_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match GenerationService::get_one_item(id, &pool).await {
        Ok(generation) => (StatusCode::OK, Json(GenerationResponse {
            success: true,
            generation: Some(generation.to_serial()),
            error: None
        })),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GenerationResponse {
                success: false,
                generation: None,
                error: Some(format!(
                    "failed to retrieve a generation with an id = {id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_generation(
    State(pool): State<PgPool>,
    Json(create): Json<CreateGeneration>,
) -> impl IntoResponse {
    match GenerationService::create_one_item(&pool, create).await {
        Ok(generation) => (StatusCode::OK, Json(GenerationResponse {
            success: true,
            generation: Some(generation.to_serial()),
            error: None
        })),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GenerationResponse {
                success: false,
                generation: None,
                error: Some(format!(
                    "failed to create a generation with the given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_generation(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateGeneration>,
) -> impl IntoResponse {
    match GenerationService::update_one_item(id, &pool, update).await {
        Ok(generation) => (StatusCode::OK, Json(GenerationResponse {
            success: true,
            generation: Some(generation.to_serial()),
            error: None
        })),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GenerationResponse {
                success: false,
                generation: None,
                error: Some(format!(
                    "failed to update the generation with id = {id} with the given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_generation(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match GenerationService::delete_one_item(id, &pool).await {
        Ok(generation) => (StatusCode::OK, Json(GenerationResponse {
            success: true,
            generation: Some(generation.to_serial()),
            error: None
        })),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GenerationResponse {
                success: false,
                generation: None,
                error: Some(format!(
                    "failed to delete the generation with id = {id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
