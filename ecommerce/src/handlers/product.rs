use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use sqlx::{query, PgPool};

use crate::{
    models::enums::category::Category,
    models::product::{
        CreateProduct, ListProductResponse, Product, ProductResponse, ProductSerializable,
    },
    utils::traits::IntoSerializable,
};

pub async fn get_products(State(pool): State<PgPool>) -> impl IntoResponse {
    let products: Vec<Product> = match sqlx::query_as(
        "select id, title, description, price, quantity, category, created_at, updated_at from products",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(products) => products,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ListProductResponse {
                    success: false,
                    products: None,
                    error: Some(format!(
                        "failed to retrieve all products due to the following error: {err:#?}"
                    )),
                }),
            )
        }
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

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(product): Json<CreateProduct>,
) -> impl IntoResponse {
    let product: Product = match query!(r#"insert into products (title, description, price, quantity, category) values ($1, $2, $3, $4, $5) returning id, title, description, price, quantity, category as "category!: Category", created_at"#, product.title, product.description, product.price, product.quantity, product.category as Category).fetch_one(&pool).await {
        Ok(product) => Product {
            id: product.id,
            title: product.title,
            description: product.description,
            price: product.price,
            quantity: product.quantity,
            created_at: product.created_at.unwrap().clone(),
            updated_at: product.created_at.unwrap(),
            category: product.category,
        },
        Err(err) => return (StatusCode::BAD_REQUEST, Json(ProductResponse {
            success: false,
            product: None,
            error: Some(format!("failed to create a new product due to the following error: {err:#?}"))
        }))
    };
    (
        StatusCode::CREATED,
        Json(ProductResponse {
            success: true,
            product: Some(product.to_serial()),
            error: None,
        }),
    )
}
