use crate::utils::time::string_to_datetime;
use crate::utils::traits::IntoSerial;
use crate::{
    models::user::{User, UserSerial},
    utils::traits::GeneralAdaptor,
};
use core::fmt;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct UserService;

#[derive(Debug, Clone, Serialize)]
pub struct ListUserResponse {
    success: bool,
    users: Option<Vec<UserSerial>>,
    error: Option<String>,
}

#[derive(Default, Deserialize)]
pub struct UserQuery {
    name: Option<String>,
    email: Option<String>,
    created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    success: bool,
    user: Option<UserSerial>,
    error: Option<String>,
}

#[derive(Default, Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String,
}
#[derive(Default, Deserialize)]
pub struct UpdateUser {
    email: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserError(String);

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to retrieve the resource due to the following error: {}",
            self.0
        )
    }
}

impl GeneralAdaptor for UserService {
    type Response = User;
    type ListResponse = Vec<User>;
    type Error = UserError;
    type Query = UserQuery;
    type CreateBody = CreateUser;
    type UpdateBody = UpdateUser;

    async fn get_all_items(
        pool: &PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query {
            UserQuery {
                name: Some(ref name),
                email: Some(ref email),
                created_at: Some(ref date),
            } => {
                let date = string_to_datetime(date, |err| {
                    UserError(format!(
                        "failed to parse the provided date into a proper date: {err:#?}"
                    ))
                })?;
                match sqlx::query!(
                    "select * from users where name = $1 and email = $2 and created_at = $3",
                    *name,
                    *email,
                    date
                )
                .fetch_all(pool)
                .await
                {
                    Ok(users) => Ok(users
                        .into_iter()
                        .map(|u| User {
                            id: u.id,
                            name: u.name,
                            email: u.email,
                            password: u.password,
                            created_at: u.created_at.unwrap(),
                            updated_at: u.updated_at.unwrap(),
                        })
                        .collect::<Vec<User>>()),
                    Err(err) => Err(UserError(format!(
                        "failed to retrieve all users due to the following error: {err:#?}"
                    ))),
                }
            }
            UserQuery {
                name: Some(ref name),
                email: Some(ref email),
                created_at: None,
            } => {
                match sqlx::query!(
                    "select * from users where name = $1 and email = $2",
                    *name,
                    *email
                )
                .fetch_all(pool)
                .await
                {
                    Ok(users) => Ok(users
                        .into_iter()
                        .map(|u| User {
                            id: u.id,
                            name: u.name,
                            email: u.email,
                            password: u.password,
                            created_at: u.created_at.unwrap(),
                            updated_at: u.updated_at.unwrap(),
                        })
                        .collect::<Vec<User>>()),
                    Err(err) => Err(UserError(format!(
                        "failed to retrieve all users due to the following error: {err:#?}"
                    ))),
                }
            }
            UserQuery {
                name: Some(ref name),
                email: None,
                created_at: None,
            } => {
                match sqlx::query!("select * from users where name = $1", *name)
                    .fetch_all(pool)
                    .await
                {
                    Ok(users) => Ok(users
                        .into_iter()
                        .map(|u| User {
                            id: u.id,
                            name: u.name,
                            email: u.email,
                            password: u.password,
                            created_at: u.created_at.unwrap(),
                            updated_at: u.updated_at.unwrap(),
                        })
                        .collect::<Vec<User>>()),
                    Err(err) => Err(UserError(format!(
                        "failed to retrieve all users due to the following error: {err:#?}"
                    ))),
                }
            }
            UserQuery {
                name: Some(ref name),
                email: None,
                created_at: Some(ref date),
            } => {
                let date = string_to_datetime(date, |err| {
                    UserError(format!(
                        "failed to parse the provided date into a proper date: {err:#?}"
                    ))
                })?;
                match sqlx::query!(
                    "select * from users where name = $1 and created_at = $2",
                    *name,
                    date
                )
                .fetch_all(pool)
                .await
                {
                    Ok(users) => Ok(users
                        .into_iter()
                        .map(|u| User {
                            id: u.id,
                            name: u.name,
                            email: u.email,
                            password: u.password,
                            created_at: u.created_at.unwrap(),
                            updated_at: u.updated_at.unwrap(),
                        })
                        .collect::<Vec<User>>()),
                    Err(err) => Err(UserError(format!(
                        "failed to retrieve all users due to the following error: {err:#?}"
                    ))),
                }
            }
            UserQuery {
                name: None,
                email: Some(ref email),
                created_at: None,
            } => {
                match sqlx::query!("select * from users where email = $1", *email,)
                    .fetch_all(pool)
                    .await
                {
                    Ok(users) => Ok(users
                        .into_iter()
                        .map(|u| User {
                            id: u.id,
                            name: u.name,
                            email: u.email,
                            password: u.password,
                            created_at: u.created_at.unwrap(),
                            updated_at: u.updated_at.unwrap(),
                        })
                        .collect::<Vec<User>>()),
                    Err(err) => Err(UserError(format!(
                        "failed to retrieve all users due to the following error: {err:#?}"
                    ))),
                }
            }
            UserQuery {
                name: None,
                email: None,
                created_at: Some(ref date),
            } => {
                let date = string_to_datetime(date, |err| {
                    UserError(format!(
                        "failed to parse the provided date into a proper date: {err:#?}"
                    ))
                })?;
                match sqlx::query!("select * from users where created_at = $1", date)
                    .fetch_all(pool)
                    .await
                {
                    Ok(users) => Ok(users
                        .into_iter()
                        .map(|u| User {
                            id: u.id,
                            name: u.name,
                            email: u.email,
                            password: u.password,
                            created_at: u.created_at.unwrap(),
                            updated_at: u.updated_at.unwrap(),
                        })
                        .collect::<Vec<User>>()),
                    Err(err) => Err(UserError(format!(
                        "failed to retrieve all users due to the following error: {err:#?}"
                    ))),
                }
            }
            _ => match sqlx::query!("select * from users",).fetch_all(pool).await {
                Ok(users) => Ok(users
                    .into_iter()
                    .map(|u| User {
                        id: u.id,
                        name: u.name,
                        email: u.email,
                        password: u.password,
                        created_at: u.created_at.unwrap(),
                        updated_at: u.updated_at.unwrap(),
                    })
                    .collect::<Vec<User>>()),
                Err(err) => Err(UserError(format!(
                    "failed to retrieve all users due to the following error: {err:#?}"
                ))),
            },
        }
    }

    async fn get_one_item(id: i32, pool: &PgPool) -> Result<Self::Response, Self::Error> {
        match sqlx::query!(
            "select id, name, email, password, created_at, updated_at from users where id = $1",
            id
        )
        .fetch_one(pool)
        .await
        {
            Ok(user) => Ok(User {
                id: user.id,
                name: user.name,
                email: user.email,
                password: user.password,
                created_at: user.created_at.unwrap(),
                updated_at: user.updated_at.unwrap(),
            }),
            Err(err) => Err(UserError(format!(
                "failed to retrieve the user with id = {id} due to the following error: {err:#?}",
            ))),
        }
    }

    async fn create_one_item(
        pool: &PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        match sqlx::query!(
            "insert into users (name, email, password) values ($1, $2, $3) returning id, name, email, password, created_at, updated_at",
            create.name,
            create.email,
            create.password
        ).fetch_one(pool).await {
            Ok(user) => Ok(User {
                id: user.id,
                name: user.name,
                email: user.email,
                password: user.password,
                created_at: user.created_at.unwrap(),
                updated_at: user.updated_at.unwrap()
            }),
            Err(err) => Err(UserError(format!(
                "failed to create a user with the given parameters due to the following error: {err:#?}"
            )))
        }
    }

    async fn update_one_item(
        id: i32,
        pool: &PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        match update {
            UpdateUser{ email: Some(ref email), password: Some(ref password) } => {
                match sqlx::query!("update users set email = $1, password = $2 where id = $3 returning id, name, email, password, created_at, updated_at", *email, *password, id).fetch_one(pool).await {
                    Ok(user) => Ok(User {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        password: user.password,
                        created_at: user.created_at.unwrap(),
                        updated_at: user.updated_at.unwrap()
                    }),
                    Err(err) => Err(UserError(format!("failed to update the user with the id = {id} due to the following error: {err:#?}")))
                }
            }
            UpdateUser{ email: Some(ref email), password: None } => {
                match sqlx::query!("update users set email = $1 where id = $2 returning id, name, email, password, created_at, updated_at", *email, id).fetch_one(pool).await {
                    Ok(user) => Ok(User {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        password: user.password,
                        created_at: user.created_at.unwrap(),
                        updated_at: user.updated_at.unwrap()
                    }),
                    Err(err) => Err(UserError(format!("failed to update the user with the id = {id} due to the following error: {err:#?}")))
                }
            }
            UpdateUser{ email: None, password: Some(ref password) } => {
                match sqlx::query!("update users set password = $1 where id = $2 returning id, name, email, password, created_at, updated_at", *password, id).fetch_one(pool).await {
                    Ok(user) => Ok(User {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        password: user.password,
                        created_at: user.created_at.unwrap(),
                        updated_at: user.updated_at.unwrap()
                    }),
                    Err(err) => Err(UserError(format!("failed to update the user with the id = {id} due to the following error: {err:#?}")))
                }
            }
            _ => sqlx::query!("select id, name, email, password, created_at, updated_at from users where id = $1", id).fetch_one(pool).await.map(|u| User {
                id: u.id,
                name: u.name,
                email: u.email,
                password: u.password,
                created_at: u.created_at.unwrap(),
                updated_at: u.updated_at.unwrap()
            }).map_err(|e| UserError(format!("failed to update the user with the id = {id} due to the following error: {e:#?}")))
        }
    }

    async fn delete_one_item(id: i32, pool: &PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!("delete from users where id = $1 returning id, name, email, password, created_at, updated_at", id).fetch_one(pool).await.map(|u| User {
            id: u.id,
            name: u.name,
            email: u.email,
            password: u.password,
            created_at: u.created_at.unwrap(),
            updated_at: u.updated_at.unwrap()
        }).map_err(|e| UserError(format!("failed to update the user with the id = {id} due to the following error: {e:#?}")))
    }
}

//handlers
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;

pub async fn get_all_users(
    State(pool): State<sqlx::PgPool>,
    query: Option<Query<UserQuery>>,
) -> impl IntoResponse {
    match UserService::get_all_items(&pool, query.map(|q| q.0).unwrap_or_default()).await {
        Ok(users) => (
            StatusCode::OK,
            Json(ListUserResponse {
                success: true,
                users: Some(
                    users
                        .into_iter()
                        .map(|u| u.to_serial())
                        .collect::<Vec<UserSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListUserResponse {
                success: false,
                users: None,
                error: Some(format!(
                    "failed to retrieve all users due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_user_by_id(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match UserService::get_one_item(id, &pool).await {
        Ok(user) => (StatusCode::OK, Json(UserResponse {
            success: true,
            user: Some(user.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(UserResponse {
            success: false,
            user: None,
            error: Some(format!("failed to retrieve a user with the id = {id} due to the following error: {err:#?}"))
        }))
    }
}

pub async fn create_user(
    State(pool): State<sqlx::PgPool>,
    Json(create): Json<CreateUser>,
) -> impl IntoResponse {
    match UserService::create_one_item(&pool, create).await {
        Ok(user) => (StatusCode::CREATED, Json(UserResponse {
            success: true,
            user: Some(user.to_serial()),
            error: None
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(UserResponse {
            success: false,
            user: None,
            error: Some(format!("failed to create a user with the given details due to the following error: {err:#?}"))
        }))
    }
}

pub async fn update_user(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateUser>,
) -> impl IntoResponse {
    match UserService::update_one_item(id, &pool, update).await {
        Ok(user) => (StatusCode::OK, Json(UserResponse {
            success: true,
            user: Some(user.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(UserResponse {
            success: false,
            user: None,
            error: Some(format!("failed to update a user with the given details due to the following error: {err:#?}"))
        }))
    }
}

pub async fn delete_user(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match UserService::delete_one_item(id, &pool).await {
        Ok(user) => (StatusCode::OK, Json(UserResponse {
            success: true,
            user: Some(user.to_serial()),
            error: None,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(UserResponse {
            success: false,
            user: None,
            error: Some(format!("failed to delete a user with the id = {id} due to the following error: {err:#?}"))
        }))
    }
}
