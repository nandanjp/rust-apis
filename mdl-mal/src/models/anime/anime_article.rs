use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::enums::anime_genres::AnimeGenres;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimeArticle {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //user
    pub article: String,
    pub show_id: ObjectId, //show
    pub genres: Vec<AnimeGenres>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}