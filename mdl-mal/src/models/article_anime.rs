use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::enums::AnimeGenres;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId,
    pub article: String,
    pub show_id: ObjectId,
    pub genres: Vec<AnimeGenres>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}