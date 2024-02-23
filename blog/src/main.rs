use crate::config::Config;
use crate::models::common::DatabaseConfig;
use crate::routes::blog::{blog_by_id, create_blog, get_all_blogs};
use crate::routes::image::{create_image, get_image_by_id, get_images};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Router};
use http::StatusCode;
use http::{header, HeaderValue};
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::time::Duration;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod config;
mod models;
mod routes;
mod utils;

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new();
    let db_config = DatabaseConfig::new(config.mongo_config());
    let mut client_options = ClientOptions::parse(db_config.uri).await.unwrap();
    client_options.connect_timeout = Some(db_config.connection_timeout);
    client_options.max_pool_size = Some(db_config.max_pool_size);
    client_options.min_pool_size = Some(db_config.min_pool_size);

    let client = Client::with_options(client_options).unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Server is healthy!" }))
        .nest(
            "/api/blog",
            Router::new()
                .route("/", get(get_all_blogs))
                .route("/", post(create_blog))
                .route("/:id", get(blog_by_id)),
        )
        .nest(
            "/api/image",
            Router::new()
                .route("/", get(get_images))
                .route("/", post(create_image))
                .route("/:id", get(get_image_by_id)),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(RequestBodyLimitLayer::new(1024))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            HeaderValue::from_static("blog"),
        ));
    let app = app.fallback(handler_404).with_state(client);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.user_config().port))
        .await
        .unwrap();
    tracing::debug!(
        "listening on {}",
        format!("0.0.0.0:{}", config.user_config().port)
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
