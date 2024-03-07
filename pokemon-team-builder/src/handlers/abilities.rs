use serde::{Deserialize, Serialize};

use crate::{
    models::{
        abilities::{Ability, AbilitySerial},
        enums::games::Game,
    },
    utils::traits::{GeneralAdaptor, IntoSerial},
};

#[derive(Debug, Clone, Serialize)]
struct AbilityResponse {
    success: bool,
    ability: Option<AbilitySerial>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ListAbilityResponse {
    success: bool,
    abilities: Option<Vec<AbilitySerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AbilityQuery {
    name: Option<String>,
    is_main_series: Option<bool>,
    generation: Option<Game>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAbility {
    name: String,
    is_main_series: Option<bool>,
    generation: Game,
    effect_entry: String,
    effect_changes: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAbility {
    effect_entry: Option<String>,
    effect_changes: Option<String>,
}

#[derive(Debug, Clone)]
struct AbilityError(String);

impl std::fmt::Display for AbilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve the ability resource due to the following error: {0:#?}",
            self.0
        )
    }
}

struct AbilityService;
impl GeneralAdaptor for AbilityService {
    type Error = AbilityError;
    type Query = AbilityQuery;
    type Response = Ability;
    type ListResponse = Vec<Ability>;
    type CreateBody = CreateAbility;
    type UpdateBody = UpdateAbility;

    async fn get_all_items(
        pool: &sqlx::PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query {
            AbilityQuery {
                generation: Some(gen),
                is_main_series: Some(is),
                ..
            } => sqlx::query!(
                r#"select id, name, generation as "generation: Game", is_main_series, effect_entry, effect_changes, created_at, updated_at from ability where generation = $1 and is_main_series = $2"#,
                gen as Game,
                is
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Ability {
                        id: a.id,
                        generation: a.generation,
                        is_main_series: a.is_main_series.unwrap(),
                        name: a.name,
                        effect_entry: a.effect_entry,
                        effect_changes: a.effect_changes,
                        created_at: a.created_at.unwrap(),
                        updated_at: a.updated_at.unwrap(),
                    })
                    .collect::<Vec<Ability>>()
            })
            .map_err(|e| {
                AbilityError(format!(
                    "failed to retrieve all abilities due to the following error: {e:#?}"
                ))
            }),
            AbilityQuery {
                generation: Some(gen),
                ..
            } => sqlx::query!(
                r#"select id, name, generation as "generation: Game", is_main_series, effect_entry, effect_changes, created_at, updated_at from ability where generation = $1"#,
                gen as Game
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Ability {
                        id: a.id,
                        generation: a.generation,
                        is_main_series: a.is_main_series.unwrap(),
                        name: a.name,
                        effect_entry: a.effect_entry,
                        effect_changes: a.effect_changes,
                        created_at: a.created_at.unwrap(),
                        updated_at: a.updated_at.unwrap(),
                    })
                    .collect::<Vec<Ability>>()
            })
            .map_err(|e| {
                AbilityError(format!(
                    "failed to retrieve all abilities due to the following error: {e:#?}"
                ))
            }),
            AbilityQuery {
                is_main_series: Some(is),
                ..
            } => sqlx::query!(r#"select id, name, generation as "generation: Game", is_main_series, effect_entry, effect_changes, created_at, updated_at from ability where is_main_series = $1"#, is)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Ability {
                            id: a.id,
                            generation: a.generation,
                            is_main_series: a.is_main_series.unwrap(),
                            name: a.name,
                            effect_entry: a.effect_entry,
                            effect_changes: a.effect_changes,
                            created_at: a.created_at.unwrap(),
                            updated_at: a.updated_at.unwrap(),
                        })
                        .collect::<Vec<Ability>>()
                })
                .map_err(|e| {
                    AbilityError(format!(
                        "failed to retrieve all abilities due to the following error: {e:#?}"
                    ))
                }),
            AbilityQuery {
                name: Some(ref name),
                ..
            } => sqlx::query!(r#"select id, name, generation as "generation: Game", is_main_series, effect_entry, effect_changes, created_at, updated_at from ability where name = $1"#, *name)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Ability {
                            id: a.id,
                            generation: a.generation,
                            is_main_series: a.is_main_series.unwrap(),
                            name: a.name,
                            effect_entry: a.effect_entry,
                            effect_changes: a.effect_changes,
                            created_at: a.created_at.unwrap(),
                            updated_at: a.updated_at.unwrap(),
                        })
                        .collect::<Vec<Ability>>()
                })
                .map_err(|e| {
                    AbilityError(format!(
                        "failed to retrieve all abilities due to the following error: {e:#?}"
                    ))
                }),
            _ => sqlx::query!(r#"select id, name, generation as "generation: Game", is_main_series, effect_entry, effect_changes, created_at, updated_at from ability"#)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Ability {
                            id: a.id,
                            generation: a.generation,
                            is_main_series: a.is_main_series.unwrap(),
                            name: a.name,
                            effect_entry: a.effect_entry,
                            effect_changes: a.effect_changes,
                            created_at: a.created_at.unwrap(),
                            updated_at: a.updated_at.unwrap(),
                        })
                        .collect::<Vec<Ability>>()
                })
                .map_err(|e| {
                    AbilityError(format!(
                        "failed to retrieve all abilities due to the following error: {e:#?}"
                    ))
                }),
        }
    }

    async fn get_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"select id, name, generation as "generation: Game", is_main_series, effect_entry, effect_changes, created_at, updated_at from ability where id = $1"#, id)
            .fetch_one(pool)
            .await
            .map(|a| Ability {
                id: a.id,
                generation: a.generation,
                is_main_series: a.is_main_series.unwrap(),
                name: a.name,
                effect_entry: a.effect_entry,
                effect_changes: a.effect_changes,
                created_at: a.created_at.unwrap(),
                updated_at: a.updated_at.unwrap(),
            })
            .map_err(|e| {
                AbilityError(format!(
                    "failed to retrieve an ability with the id = {id} due to the following error: {e:#?}"
                ))
            })
    }

    async fn create_one_item(
        pool: &sqlx::PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query!(
            r#"insert into ability (name, is_main_series, generation, effect_entry, effect_changes) values ($1, $2, $3, $4, $5) returning id, name, is_main_series, generation as "generation: Game", effect_entry, effect_changes, created_at, updated_at"#,
            create.name,
            create.is_main_series.unwrap_or_default(),
            create.generation as Game,
            create.effect_entry,
            create.effect_changes
        ).fetch_one(pool).await.map(|a| Ability {
            id: a.id,
            generation: a.generation,
            is_main_series: a.is_main_series.unwrap(),
            name: a.name,
            effect_entry: a.effect_entry,
            effect_changes: a.effect_changes,
            created_at: a.created_at.unwrap(),
            updated_at: a.updated_at.unwrap(),
        }).map_err(|e| {
            AbilityError(format!(
                "failed to create an ability with the provided details due to the following error: {e:#?}"
            ))
        })
    }

    async fn update_one_item(
        id: i32,
        pool: &sqlx::PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        match update {
            UpdateAbility {effect_changes: Some(ec), effect_entry: Some(ee)} => {
                sqlx::query!(
                    r#"update ability set effect_changes = $1, effect_entry = $2 where id = $3 returning id, name, is_main_series, effect_entry, effect_changes, generation as "generation: Game", created_at, updated_at"#,
                    ec,
                    ee,
                    id
                ).fetch_one(pool).await.map(|a| Ability {
                    id: a.id,
                    generation: a.generation,
                    is_main_series: a.is_main_series.unwrap(),
                    name: a.name,
                    effect_entry: a.effect_entry,
                    effect_changes: a.effect_changes,
                    created_at: a.created_at.unwrap(),
                    updated_at: a.updated_at.unwrap(),
                }).map_err(|e| {
                    AbilityError(format!(
                        "failed to update the ability with id = {id} due to the following error: {e:#?}"
                    ))
                })
            }
            UpdateAbility {effect_entry: Some(ee), ..} => {
                sqlx::query!(
                    r#"update ability set effect_entry = $1 where id = $2 returning id, name, is_main_series, effect_entry, effect_changes, generation as "generation: Game", created_at, updated_at"#,
                    ee,
                    id
                ).fetch_one(pool).await.map(|a| Ability {
                    id: a.id,
                    generation: a.generation,
                    is_main_series: a.is_main_series.unwrap(),
                    name: a.name,
                    effect_entry: a.effect_entry,
                    effect_changes: a.effect_changes,
                    created_at: a.created_at.unwrap(),
                    updated_at: a.updated_at.unwrap(),
                }).map_err(|e| {
                    AbilityError(format!(
                        "failed to update the ability with id = {id} due to the following error: {e:#?}"
                    ))
                })
            }
            UpdateAbility {effect_changes: Some(ec), ..} => {
                sqlx::query!(
                    r#"update ability set effect_changes = $1 where id = $2 returning id, name, is_main_series, effect_entry, effect_changes, generation as "generation: Game", created_at, updated_at"#,
                    ec,
                    id
                ).fetch_one(pool).await.map(|a| Ability {
                    id: a.id,
                    generation: a.generation,
                    is_main_series: a.is_main_series.unwrap(),
                    name: a.name,
                    effect_entry: a.effect_entry,
                    effect_changes: a.effect_changes,
                    created_at: a.created_at.unwrap(),
                    updated_at: a.updated_at.unwrap(),
                }).map_err(|e| {
                    AbilityError(format!(
                        "failed to update the ability with id = {id} due to the following error: {e:#?}"
                    ))
                })
            }
            _ => {
                sqlx::query!(
                    r#"select id, name, is_main_series, generation as "generation: Game", effect_entry, effect_changes, created_at, updated_at from ability where id = $1"#,
                    id
                ).fetch_one(pool).await.map(|a| Ability {
                    id: a.id,
                    generation: a.generation,
                    is_main_series: a.is_main_series.unwrap(),
                    name: a.name,
                    effect_entry: a.effect_entry,
                    effect_changes: a.effect_changes,
                    created_at: a.created_at.unwrap(),
                    updated_at: a.updated_at.unwrap(),
                }).map_err(|e| {
                    AbilityError(format!(
                        "failed to update the ability with id = {id} due to the following error: {e:#?}"
                    ))
                })
            }
        }
    }

    async fn delete_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"delete from ability where id = $1 returning id, name, is_main_series, generation as "generation: Game", effect_entry, effect_changes, created_at, updated_at"#, id).fetch_one(pool).await.map(|a| Ability {
            id: a.id,
            generation: a.generation,
            is_main_series: a.is_main_series.unwrap(),
            name: a.name,
            effect_entry: a.effect_entry,
            effect_changes: a.effect_changes,
            created_at: a.created_at.unwrap(),
            updated_at: a.updated_at.unwrap(),
        }).map_err(|e| AbilityError(format!(
            "failed to delete the ability with id = {id} due to the following error: {e:#?}"
        )))
    }
}

use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_abilities(
    State(pool): State<PgPool>,
    query: Option<Query<AbilityQuery>>,
) -> impl IntoResponse {
    match AbilityService::get_all_items(&pool, query.unwrap_or_default().0).await {
        Ok(abilities) => (
            StatusCode::OK,
            Json(ListAbilityResponse {
                success: true,
                abilities: Some(
                    abilities
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<AbilitySerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListAbilityResponse {
                success: false,
                abilities: None,
                error: Some(format!(
                    "failed to retrieve all abilities due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_ability_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match AbilityService::get_one_item(id, &pool).await {
        Ok(abilities) => (
            StatusCode::OK,
            Json(AbilityResponse {
                success: true,
                ability: Some(abilities.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AbilityResponse {
                success: false,
                ability: None,
                error: Some(format!(
                    "failed to get an ability with id = {id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_ability(
    State(pool): State<PgPool>,
    Json(create): Json<CreateAbility>,
) -> impl IntoResponse {
    match AbilityService::create_one_item(&pool, create).await {
        Ok(abilities) => (
            StatusCode::OK,
            Json(AbilityResponse {
                success: true,
                ability: Some(abilities.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AbilityResponse {
                success: false,
                ability: None,
                error: Some(format!(
                    "failed to create a new ability due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_ability(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateAbility>,
) -> impl IntoResponse {
    match AbilityService::update_one_item(id, &pool, update).await {
        Ok(abilities) => (
            StatusCode::OK,
            Json(AbilityResponse {
                success: true,
                ability: Some(abilities.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AbilityResponse {
                success: false,
                ability: None,
                error: Some(format!(
                    "failed to update the ability with id = {id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_ability(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match AbilityService::delete_one_item(id, &pool).await {
        Ok(abilities) => (
            StatusCode::OK,
            Json(AbilityResponse {
                success: true,
                ability: Some(abilities.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AbilityResponse {
                success: false,
                ability: None,
                error: Some(format!(
                    "failed to delete the ability with id = {id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
