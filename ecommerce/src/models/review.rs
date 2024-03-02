use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerializable;

#[derive(FromRow)]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub rating: f32,
    pub review: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerializable<ReviewSerializable> for Review {
    fn to_serial(self) -> ReviewSerializable {
        ReviewSerializable {
            review_id: self.id,
            user_id: self.user_id,
            product_id: self.product_id,
            rating: self.rating,
            review: self.review,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ReviewSerializable {
    pub review_id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub rating: f32,
    pub review: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateReview {
    pub user: String,
    pub email: String,
    pub product: String,
    pub rating: f64,
    pub review: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateReview {
    pub product: Option<String>,
    pub rating: Option<f64>,
    pub review: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ReviewResponse {
    pub success: bool,
    pub review: Option<ReviewSerializable>,
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ListReviewResponse {
    pub success: bool,
    pub reviews: Option<Vec<ReviewSerializable>>,
    pub error: Option<String>,
}

impl CreateReview {}
impl UpdateReview {}
