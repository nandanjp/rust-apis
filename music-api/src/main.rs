use dotenv;

#[tokio::main]
async fn main() {
    dotenv::ok();

    let database = std::env::var("DATABASE_URL").expect("expected a database url to be provided however none was defined".to_string());
    let pool = PgPoolOptions::new().max_connections(3).acquire_timeout(std::time::Duration::from_secs(3)).connect(&database).await.expect("failed to connect to the database with the provided url");
}
