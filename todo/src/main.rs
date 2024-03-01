use std::time::Duration;
use axum::routing::post;
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn internal_error<E>(err: E) -> (StatusCode, Json<Message>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Message {
            message: err.to_string(),
        }),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    message: String,
}

async fn get_todos(State(pool): State<sqlx::postgres::PgPool>) -> impl IntoResponse {
    let result = sqlx::query_scalar!("select task from todo")
        .fetch_all(&pool)
        .await;
    match result {
        Ok(val) => (
            StatusCode::OK,
            Json(Message {
                message: format!("successfully got a result: {:#?}", val),
            }),
        ),
        Err(err) => internal_error(err),
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CreateTodo {
    task: String,
}

async fn create_todo(
    State(pool): State<sqlx::postgres::PgPool>,
    Json(todo): Json<CreateTodo>,
) -> impl IntoResponse {
    match sqlx::query_scalar!(
        "insert into todo ( task, completed ) values ( $1, $2 ) returning id",
        todo.task,
        false,
    )
    .fetch_one(&pool)
    .await
    {
        Ok(res) => (
            StatusCode::OK,
            Json(Message {
                message: format!("successfully created a todo item: {:#?}", res),
            }),
        ),
        Err(err) => internal_error(err),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:17012004@127.0.0.1:5433/todo".to_string());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(|| async { "health check passed successfully" }))
        .route("/todo", get(get_todos))
        .route("/todo", post(create_todo));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app.with_state(pool).into_make_service())
        .await
        .unwrap()
}
