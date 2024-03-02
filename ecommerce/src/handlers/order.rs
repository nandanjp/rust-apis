use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::order::{ListOrderResponse, Order, OrderSerializable},
    utils::traits::IntoSerializable,
};

pub async fn get_orders(State(pool): State<PgPool>) -> impl IntoResponse {
    let orders: Vec<Order> =
        match sqlx::query_as("select id, user_id, destination, created_at from orders")
            .fetch_all(&pool)
            .await
        {
            Ok(orders) => orders,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ListOrderResponse {
                        success: false,
                        orders: None,
                        error: Some(format!(
                            "failed to get all orders due to the following error: {err:#?}"
                        )),
                    }),
                )
            }
        };
    let orders = orders
        .into_iter()
        .map(|o| o.to_serial())
        .collect::<Vec<OrderSerializable>>();
    (
        StatusCode::OK,
        Json(ListOrderResponse {
            success: true,
            orders: Some(orders),
            error: None,
        }),
    )
}
