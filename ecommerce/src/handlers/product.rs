use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::product::{ListProductResponse, Product, ProductSerializable},
    utils::traits::IntoSerializable,
};

pub async fn get_products(State(pool): State<PgPool>) -> impl IntoResponse {
    let products: Vec<Product> = match sqlx::query_as("select id, title, desrciption, price, quantity_available, category, created_at from products").fetch_all(&pool).await {
        Ok(products) => products,
        Err(err) => return (StatusCode::BAD_REQUEST, Json(ListProductResponse {
            success: false,
            products: None,
            error: Some(format!("failed to retrieve all products due to the following error: {err:#?}"))
        }))
    };

    let products = products
        .into_iter()
        .map(|p| p.to_serial())
        .collect::<Vec<ProductSerializable>>();

    (
        StatusCode::OK,
        Json(ListProductResponse {
            success: true,
            products: Some(products),
            error: None,
        }),
    )
}
