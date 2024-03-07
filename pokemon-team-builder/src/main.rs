use std::time::Duration;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use handlers::{
    abilities::{
        create_ability, delete_ability, get_ability_by_id, get_all_abilities, update_ability,
    },
    games::{
        create_generation, delete_generation, get_all_generations, get_generation_by_id,
        update_generation,
    },
    users::{create_user, delete_user, get_all_users, get_user_by_id, update_user},
};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "pokemon_team_builder=debug,axum=debug,tower_http=debug,".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:17012004@127.0.0.1:5432/pokemon_team".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_str)
        .await
        .expect(&format!(
            "failed to connect to the database using the the provided connection: {db_str}"
        ));

    // match sqlx::migrate!().run(&pool).await {
    //     Ok(_) => tracing::debug!("Successfully ran the most recent migration!"),
    //     Err(err) => tracing::debug!(
    //         "Failed to run the most recent migration due to the following error: {err:#?}"
    //     ),
    // }
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u32>()
        .expect("failed to retrieve a valid port");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .expect("failed to retrieve a tcp listener: could not start up a server on the given port");

    tracing::debug!("Now listening on port {port}");

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "Server is healthy and running!" }),
        )
        .nest(
            "/api",
            Router::new()
                .nest("/pokemon", Router::new())
                .nest(
                    "/ability",
                    Router::new()
                        .route("/", get(get_all_abilities))
                        .route("/", post(create_ability))
                        .route("/:id", get(get_ability_by_id))
                        .route("/:id", put(update_ability))
                        .route("/:id", delete(delete_ability)),
                )
                .nest("/move", Router::new())
                .nest(
                    "/game",
                    Router::new()
                        .route("/", get(get_all_generations))
                        .route("/", post(create_generation))
                        .route("/:id", get(get_generation_by_id))
                        .route("/:id", put(update_generation))
                        .route("/:id", delete(delete_generation)),
                )
                .nest(
                    "/user",
                    Router::new()
                        .route("/", get(get_all_users))
                        .route("/", post(create_user))
                        .route("/:id", get(get_user_by_id))
                        .route("/:id", put(update_user))
                        .route("/:id", delete(delete_user)),
                ),
        )
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(
            10,
        )))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods([http::Method::GET, http::Method::POST, http::Method::DELETE])
                .allow_origin(tower_http::cors::Any),
        );

    axum::serve(listener, app.with_state(pool).into_make_service())
        .await
        .expect("failed to serve the axum server on the provided tcp listener")
}
