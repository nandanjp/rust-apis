mod config;
mod handlers;

use handlers::authenticate::{authorize, get_user};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
pub struct AppState {
    pub jwt_secret: String,
    pub client_secret: String,
    pub client_id: String,
    pub pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_jwt=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::new().expect("failed to parse the expected configuration");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&config.db_url)
        .await
        .expect(&format!(
            "failed to connect to the database defined under the provided database_url = {}",
            &config.db_url
        ));

    let app = axum::Router::new()
        .route(
            "/health",
            axum::routing::get(|| async { "Server is healthy and listening on the given port" }),
        )
        .route("/api/auth", axum::routing::get(authorize))
        .route("/api/user", axum::routing::get(get_user))
        .with_state(AppState {
            jwt_secret: config.jwt_token,
            client_id: config.spotify_client_id,
            client_secret: config.spotify_secret,
            pool,
        })
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", config.port))
        .await
        .expect(&format!(
            "failed to bind a tcp listener on port = {}",
            config.port
        ));

    tracing::debug!("Now listening on port 4000");

    axum::serve(listener, app)
        .await
        .expect("failed to sever the app on the provided listener");
}
