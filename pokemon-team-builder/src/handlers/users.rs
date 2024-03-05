use core::fmt;

use crate::{
    models::user::{User, UserSerial},
    utils::traits::{GeneralAdaptor, GeneralService},
};
use axum::extract::{Json, Query, State};

struct UserService;
struct ListUserResponse {
    success: bool,
    users: Option<Vec<UserSerial>>,
    error: Option<String>,
}
struct UserQuery {}
struct UserResponse {
    success: bool,
    user: Option<UserSerial>,
    error: Option<String>,
}
struct CreateUser {}
struct UpdateUser {}

#[derive(Debug, Clone)]
struct UserError(String);

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

    async fn get_all_items(query: Self::Query) -> Result<Self::ListResponse, Self::Error> {
        todo!()
    }

    async fn get_one_item() -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn create_one_item(create: Self::CreateBody) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn update_one_item(update: Self::UpdateBody) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    async fn delete_one_item() -> Result<Self::Response, Self::Error> {
        todo!()
    }
}

impl GeneralService for UserService {
    type ListResponse = ListUserResponse;
    type Response = UserResponse;
    type QueryAll = UserQuery;
    type CreateBody = CreateUser;
    type UpdateBody = UpdateUser;

    async fn get_all(
        State(pool): axum::extract::State<sqlx::PgPool>,
        Query(query): axum::extract::Query<Self::QueryAll>,
    ) -> (http::StatusCode, axum::Json<Self::ListResponse>) {
        todo!()
    }

    async fn get_one(
        State(pool): axum::extract::State<sqlx::PgPool>,
        Query(id): axum::extract::Query<i32>,
    ) -> (http::StatusCode, axum::Json<Self::Response>) {
        todo!()
    }

    async fn create_one(
        State(pool): axum::extract::State<sqlx::PgPool>,
        Json(create): axum::Json<Self::CreateBody>,
    ) -> (http::StatusCode, axum::Json<Self::Response>) {
        todo!()
    }

    async fn update_one(
        State(pool): axum::extract::State<sqlx::PgPool>,
        Query(id): axum::extract::Query<i32>,
        Json(update): axum::Json<Self::UpdateBody>,
    ) -> (http::StatusCode, axum::Json<Self::Response>) {
        todo!()
    }

    async fn delete_one(
        State(pool): axum::extract::State<sqlx::PgPool>,
        Query(id): axum::extract::Query<i32>,
    ) -> (http::StatusCode, axum::Json<Self::Response>) {
        todo!()
    }
}
