#[tokio::main]
async fn main() {
   dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("failed to find a database url");
    let port = std::env::var("PORT").map(|p| p.parse::<u32>().unwrap()).expect("failed to find a valid port defined");

    let pool = sqlx::postgres::PgPoolOptions::new().max_connections(3).acquire_timeout(std::time::Duration::from_secs(5)).connect(&db_url).await.expect(&format!("failed to connect to the database defined at the given address: {}", db_url));
    let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", port)).await.expect(&format!("failed to retrieve a tcp listener defined at the given port: {}", port));
    let app = axum::Router::new().route("/health", axum::routing::get(|| async { "Server is healthy and listening" }));

    axum::serve(listener, app.with_state(pool).into_make_service()).await.expect("failed to serve the given app using the provided listener");
}
