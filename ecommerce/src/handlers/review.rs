use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::review::{ListReviewResponse, Review, ReviewSerializable},
    utils::traits::IntoSerializable,
};

pub async fn get_reviews(State(pool): State<PgPool>) -> impl IntoResponse {
    let reviews: Vec<Review> = match sqlx::query_as(
        "select id, user_id, product_id, rating, review, created_at from reviews",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(reviews) => reviews,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ListReviewResponse {
                    success: false,
                    reviews: None,
                    error: Some(format!(
                        "failed to get all reviews due to the following error: {err:#?}"
                    )),
                }),
            )
        }
    };
    let reviews = reviews
        .into_iter()
        .map(|r| r.to_serial())
        .collect::<Vec<ReviewSerializable>>();
    (
        StatusCode::OK,
        Json(ListReviewResponse {
            success: true,
            reviews: Some(reviews),
            error: None,
        }),
    )
}
