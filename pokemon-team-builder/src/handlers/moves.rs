use serde::{Deserialize, Serialize};

use crate::{
    models::{
        enums::types::Type,
        moves::{Move, MoveSerial},
    },
    utils::traits::{GeneralAdaptor, IntoSerial},
};

#[derive(Debug, Clone, Serialize)]
struct MoveResponse {
    success: bool,
    #[serde(rename = "move")]
    move_: Option<MoveSerial>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ListMoveResponse {
    success: bool,
    moves: Option<Vec<MoveSerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MoveQuery {
    priority: Option<i32>,
    power: Option<i32>,
    #[serde(rename = "type")]
    type_: Option<Type>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMove {
    name: String,
    accuracy: Option<i32>,
    effect_change: Option<i32>,
    pp: Option<i32>,
    priority: i32,
    power: i32,
    #[serde(rename = "type")]
    type_: Type,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMove {
    name: Option<String>,
    accuracy: Option<i32>,
    effect_change: Option<i32>,
    pp: Option<i32>,
    priority: Option<i32>,
    power: Option<i32>,
    #[serde(rename = "type")]
    type_: Option<Type>,
}

#[derive(Debug, Clone)]
struct MoveError(String);
impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to operate on the move resource due to the following error: {:#?}",
            self.0
        )
    }
}

struct MoveService;
impl GeneralAdaptor for MoveService {
    type Error = MoveError;
    type UpdateBody = UpdateMove;
    type CreateBody = CreateMove;
    type Response = Move;
    type ListResponse = Vec<Move>;
    type Query = MoveQuery;

    async fn get_all_items(
        pool: &sqlx::PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query {
            MoveQuery { power: Some(pow), priority: Some(priority), type_: Some(t) } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where power = $1 and priority = $2 and type = $3"#, pow, priority, t as Type).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            MoveQuery { power: Some(pow), priority: Some(priority), .. } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where power = $1 and priority = $2"#, pow, priority).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            MoveQuery { power: Some(pow), type_: Some(t), .. } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where power = $1 and type = $2"#, pow, t as Type).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            MoveQuery { priority: Some(priority), type_: Some(t), .. } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where priority = $1 and type = $2"#, priority, t as Type).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            MoveQuery { power: Some(pow), .. } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where power = $1"#, pow).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            MoveQuery { priority: Some(priority), .. } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where priority = $1"#, priority).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            MoveQuery { type_: Some(t), .. } => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where type = $1"#, t as Type).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            },
            _ => {
                sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move"#).fetch_all(pool).await.map(|moves| {
                    moves
                        .into_iter()
                        .map(|m| Move {
                            id: m.id,
                            name: m.name,
                            priority: m.priority,
                            power: m.power,
                            accuracy: m.accuracy,
                            effect_change: m.effect_change,
                            pp: m.pp,
                            type_: m.type_,
                            created_at: m.created_at.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        })
                        .collect::<Vec<Move>>()
                })
                .map_err(|e| {
                    MoveError(format!(
                        "failed to retrieve all moves due to the following error: {e:#?}"
                    ))
                })
            }
        }
    }

    async fn get_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"select id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at from move where id = $1"#, id).fetch_one(pool).await.map(|m| Move {
            id: m.id,
            name: m.name,
            priority: m.priority,
            power: m.power,
            accuracy: m.accuracy,
            effect_change: m.effect_change,
            pp: m.pp,
            type_: m.type_,
            created_at: m.created_at.unwrap(),
            updated_at: m.updated_at.unwrap(),
        }).map_err(|e| {
            MoveError(format!(
                "failed to find a move with the id = {id} due to the following error: {e:#?}"
            ))
        })
    }

    async fn create_one_item(
        pool: &sqlx::PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"insert into move (name, accuracy, effect_change, pp, priority, power, type) values ($1, $2, $3, $4, $5, $6, $7) returning id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at"#, create.name, create.accuracy.unwrap_or_default(), create.effect_change, create.pp.unwrap_or_default(), create.priority, create.power, create.type_ as Type).fetch_one(pool).await.map(|m| Move {
            id: m.id,
            name: m.name,
            priority: m.priority,
            power: m.power,
            accuracy: m.accuracy,
            effect_change: m.effect_change,
            pp: m.pp,
            type_: m.type_,
            created_at: m.created_at.unwrap(),
            updated_at: m.updated_at.unwrap(),
        }).map_err(|e| {
            MoveError(format!(
                "failed to create a move with the provided details due to the following error: {e:#?}"
            ))
        })
    }

    async fn update_one_item(
        id: i32,
        pool: &sqlx::PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        let prev = Self::get_one_item(id, pool).await?;
        sqlx::query!(r#"update move set name = $1, accuracy = $2, effect_change = $3, pp = $4, priority = $5, power = $6, type = $7 where id = $8 returning id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at"#, update.name.unwrap_or(prev.name), update.accuracy.unwrap_or(prev.accuracy.unwrap()), update.effect_change.unwrap_or(prev.effect_change.unwrap()), update.pp.unwrap_or(prev.pp.unwrap()), update.priority.unwrap_or(prev.priority), update.power.unwrap_or(prev.power.unwrap()), update.type_.unwrap_or(prev.type_) as Type, id).fetch_one(pool).await.map(|m| Move {
            id: m.id,
            name: m.name,
            priority: m.priority,
            power: m.power,
            accuracy: m.accuracy,
            effect_change: m.effect_change,
            pp: m.pp,
            type_: m.type_,
            created_at: m.created_at.unwrap(),
            updated_at: m.updated_at.unwrap(),
        }).map_err(|e| {
            MoveError(format!(
                "failed to update the move with id = {id} due to the following error: {e:#?}"
            ))
        })
    }

    async fn delete_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"delete from move where id = $1 returning id, name, accuracy, effect_change, pp, priority, power, type as "type_: Type", created_at, updated_at"#, id)
            .fetch_one(pool)
            .await
            .map(|m| Move {
                id: m.id,
                name: m.name,
                priority: m.priority,
                power: m.power,
                accuracy: m.accuracy,
                effect_change: m.effect_change,
                pp: m.pp,
                type_: m.type_,
                created_at: m.created_at.unwrap(),
                updated_at: m.updated_at.unwrap(),
            })
            .map_err(|e| {
                MoveError(format!(
                    "failed to delete the move with id = {id} due to the following error: {e:#?}"
                ))
            })
    }
}

use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_moves(
    State(pool): State<PgPool>,
    query: Option<Query<MoveQuery>>,
) -> impl IntoResponse {
    match MoveService::get_all_items(&pool, query.unwrap_or_default().0).await {
        Ok(moves) => (
            StatusCode::OK,
            Json(ListMoveResponse {
                success: true,
                moves: Some(
                    moves
                        .into_iter()
                        .map(|m| m.to_serial())
                        .collect::<Vec<MoveSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListMoveResponse {
                success: false,
                moves: None,
                error: Some(format!(
                    "failed to retrieve all moves due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_move_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match MoveService::get_one_item(id, &pool).await {
        Ok(move_) => (StatusCode::OK, Json(MoveResponse {
            success: true,
            move_: Some(move_.to_serial()),
            error: None
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(MoveResponse {
            success: false,
            move_: None,
            error: Some(format!("failed to retrieve a move with the id = {id} due to the following error: {err:#?}"))
        }))
    }
}

pub async fn create_move(
    State(pool): State<PgPool>,
    Json(create): Json<CreateMove>,
) -> impl IntoResponse {
    match MoveService::create_one_item(&pool, create).await {
        Ok(move_) => (StatusCode::CREATED, Json(MoveResponse {
            success: true,
            move_: Some(move_.to_serial()),
            error: None
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(MoveResponse {
            success: false,
            move_: None,
            error: Some(format!("failed to create a move with the provided details due to the following error: {err:#?}"))
        }))
    }
}

pub async fn update_move(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateMove>,
) -> impl IntoResponse {
    match MoveService::update_one_item(id, &pool, update).await {
        Ok(move_) => (StatusCode::OK, Json(MoveResponse {
            success: true,
            move_: Some(move_.to_serial()),
            error: None
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(MoveResponse {
            success: false,
            move_: None,
            error: Some(format!("failed to update a move with the id = {id} and the provided details due to the following error: {err:#?}"))
        }))
    }
}

pub async fn delete_move(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match MoveService::delete_one_item(id, &pool).await {
        Ok(move_) => (StatusCode::OK, Json(MoveResponse {
            success: true,
            move_: Some(move_.to_serial()),
            error: None
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(MoveResponse {
            success: false,
            move_: None,
            error: Some(format!("failed to delete a move with the id = {id} due to the following error: {err:#?}"))
        }))
    }
}
