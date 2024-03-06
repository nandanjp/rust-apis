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

pub trait GeneralAdaptor {
    type Response;
    type ListResponse;
    type Error;
    type CreateBody;
    type UpdateBody;
    type Query;

    async fn get_all_items(
        pool: &PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error>;
    async fn get_one_item(id: i32, pool: &PgPool) -> Result<Self::Response, Self::Error>;
    async fn create_one_item(
        pool: &PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error>;
    async fn update_one_item(
        id: i32,
        pool: &PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error>;
    async fn delete_one_item(id: i32, pool: &PgPool) -> Result<Self::Response, Self::Error>;
}
