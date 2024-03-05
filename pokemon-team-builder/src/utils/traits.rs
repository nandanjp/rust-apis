use axum::extract::{Json, Query, State};
use http::StatusCode;
use sqlx::PgPool;

pub trait SerDeserEnum {
    type Error;
    fn to_str(&self) -> &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait IntoSerial {
    type Serial;
    fn to_serial(&self) -> Self::Serial;
}

pub trait GeneralService {
    type ListResponse;
    type Response;
    type CreateBody;
    type UpdateBody;
    type QueryAll;

    async fn get_all(
        pool: State<PgPool>,
        query: Query<Self::QueryAll>,
    ) -> (StatusCode, Json<Self::ListResponse>);
    async fn get_one(pool: State<PgPool>, id: Query<i32>) -> (StatusCode, Json<Self::Response>);
    async fn create_one(
        pool: State<PgPool>,
        create: Json<Self::CreateBody>,
    ) -> (StatusCode, Json<Self::Response>);
    async fn update_one(
        pool: State<PgPool>,
        id: Query<i32>,
        update: Json<Self::UpdateBody>,
    ) -> (StatusCode, Json<Self::Response>);
    async fn delete_one(pool: State<PgPool>, id: Query<i32>) -> (StatusCode, Json<Self::Response>);
}

pub trait GeneralAdaptor {
    type Response;
    type ListResponse;
    type Error;
    type CreateBody;
    type UpdateBody;
    type Query;

    async fn get_all_items(query: Self::Query) -> Result<Self::ListResponse, Self::Error>;
    async fn get_one_item() -> Result<Self::Response, Self::Error>;
    async fn create_one_item(create: Self::CreateBody) -> Result<Self::Response, Self::Error>;
    async fn update_one_item(update: Self::UpdateBody) -> Result<Self::Response, Self::Error>;
    async fn delete_one_item() -> Result<Self::Response, Self::Error>;
}
