use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::{
        enums::user_role::UserRole,
        order::{CreateOrder, ListOrderResponse, Order, OrderResponse, OrderSerializable},
        user::User,
    },
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

pub async fn create_order(
    State(pool): State<PgPool>,
    Json(order): Json<CreateOrder>,
) -> impl IntoResponse {
    let user: User = match sqlx::query!(r#"select id, username, email, password, address, users_role as "users_role!: UserRole", created_at, update_at from users where email = $1"#, order.customer_email).fetch_one(&pool).await {
        Ok(user) => User {
            id: user.id,
            username: user.username,
            email: user.email,
            address: user.address,
            created_at: user.created_at.unwrap().clone(),
            updated_at: user.update_at.unwrap(),
            password: user.password,
            users_role: user.users_role
        },
        Err(err) => return (
            StatusCode::NOT_FOUND, Json(OrderResponse {
                success: false,
                order: None,
                error: Some(format!("failed to find a user with the provided email and thus could not generate an order: {err:#?}"))
            })
        )
    };

    if let UserRole::Admin = user.users_role {
        return (StatusCode::NOT_FOUND, Json(OrderResponse {
         success: false,
      order: None,
   error: Some(format!("sorry, at the moment admin's should not be making an order.... Try using a customer account"))
        }));
    }

    let order = match sqlx::query!("insert into orders (user_id, destination) values ($1, $2) returning id, user_id, destination, created_at", user.id, order.destination).fetch_one(&pool).await {
        Ok(order) => Order {
            id: order.id,
            user_id: order.user_id,
            destination: order.destination,
            created_at: order.created_at.unwrap().clone(),
            updated_at: order.created_at.unwrap()
        },
        Err(err) => return (
            StatusCode::BAD_REQUEST, Json(OrderResponse {
                success: false,
                order: None,
                error: Some(format!("failed to create a new order due to the following error: {err:#?}"))
            })
        )
    };

    (
        StatusCode::CREATED,
        Json(OrderResponse {
            success: true,
            order: Some(order.to_serial()),
            error: None,
        }),
    )
}
