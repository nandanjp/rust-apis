use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use sqlx::{postgres::PgPool, query, query_as};

use crate::{
    models::{
        enums::user_role::UserRole,
        user::{CreateUser, ListUserResponse, User, UserResponse, UserSerializable},
    },
    utils::traits::IntoSerializable,
};

pub async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let users: Vec<User> = match query_as("select id, username, email, user_password, user_address, users_role, created_at, update_at from users")
    .fetch_all(&pool)
    .await
    {
        Ok(users) => users,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ListUserResponse {
                    success: false,
                    users: None,
                    error: Some(format!(
                        "failed to retrieve users due to the following error: {err:#?}"
                    )),
                }),
            )
        }
    };
    let users = users
        .into_iter()
        .map(|u| u.to_serial())
        .collect::<Vec<UserSerializable>>();
    (
        StatusCode::OK,
        Json(ListUserResponse {
            success: true,
            users: Some(users),
            error: None,
        }),
    )
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
    let new_pass = match bcrypt::hash(user.password, 10) {
        Ok(pass) => pass,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UserResponse {
                    success: false,
                    user: None,
                    error: Some(format!(
                        "failed to hash the user's password into something unique: {err:#?}"
                    )),
                }),
            )
        }
    };

    let user: User = match query!(
        r#"insert into users (username, email, user_password, user_address, users_role) values ($1, $2, $3, $4, $5) returning id, username, email, user_password, user_address, created_at, users_role as "users_role!: UserRole""#,
        user.username,
        user.email,
        new_pass,
        user.address,
        user.role as UserRole
    )
    .fetch_one(&pool)
    .await
    {
        Ok(user) => User {
            id: user.id,
            username: user.username,
            email: user.email,
            user_password: user.user_password,
            user_address: user.user_address,
            users_role: user.users_role,
            created_at: user.created_at.unwrap(),
            update_at: user.created_at.unwrap(),
        },
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UserResponse {
                    success: false,
                    user: None,
                    error: Some(format!(
                        "failed to create a new user due to the following error: {err:#?}"
                    )),
                }),
            )
        }
    };

    (
        StatusCode::CREATED,
        Json(UserResponse {
            success: true,
            user: Some(user.to_serial()),
            error: None,
        }),
    )
}
