use std::time::Duration;
use axum::Router;
use axum::routing::get;
use http::StatusCode;
use http::{header, HeaderValue};
use mongodb::Client;
use mongodb::options::ClientOptions;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::Config;
use crate::models::database::DatabaseConfig;

mod utils;
mod models;
mod handlers;
mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            })
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new();
    let database_config = DatabaseConfig::new(config.get_mongo_config());
    let mut options = ClientOptions::parse(config.get_mongo_config().get_uri()).await.expect("Failed to parse the mongo uri correctly");
    options.connect_timeout = Some(database_config.connection_timeout);
    options.max_pool_size = Some(database_config.max_pool_size);
    options.max_pool_size = Some(database_config.min_pool_size);
    let client = Client::with_options(options).unwrap();
    tracing::debug!("Successfully connected to the mongo database");

    let app = Router::new()
        .route("/", get(|| async { "Server is healthy!" }))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(RequestBodyLimitLayer::new(1024))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            HeaderValue::from_static("blog"),
        ));
    let app = app.fallback(|| async { (StatusCode::NOT_FOUND, "nothing to see here")}).with_state(client);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.get_user_config().get_port())).await.unwrap();
    tracing::debug!(
        "listening on {}",
        format!("0.0.0.0:{}", config.get_user_config().get_port())
    );
    axum::serve(listener, app.into_make_service()).await.unwrap()
}
