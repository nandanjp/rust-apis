use std::time::Duration;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod utils;

use handlers::{
    order::{create_order, get_orders},
    product::{create_product, get_products},
    review::get_reviews,
    user::{create_user, get_users},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ecommerce_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:17012004@127.0.0.1:5432/ecommerce".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_str)
        .await
        .expect("failed to connect to the postgres ecommerce database");
    let app = Router::new()
        .route(
            "/health_check",
            get(|| async { "Server is Healthy and Running" }),
        )
        .nest(
            "/api",
            Router::new()
                .nest("/auth", Router::new())
                .nest(
                    "/user",
                    Router::new()
                        .route("/", get(get_users))
                        .route("/", post(create_user)),
                )
                .nest(
                    "/product",
                    Router::new()
                        .route("/", get(get_products))
                        .route("/", post(create_product)),
                )
                .nest(
                    "/order",
                    Router::new()
                        .route("/", get(get_orders))
                        .route("/", post(create_order)),
                )
                .nest("/review", Router::new().route("/", get(get_reviews))),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind a tcp listener to the port 5000");
    tracing::debug!("Server is now listening on port :5000");

    axum::serve(listener, app.with_state(pool).into_make_service())
        .await
        .unwrap();
}
