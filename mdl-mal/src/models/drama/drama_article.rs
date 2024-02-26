use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::enums::drama_genres::DramaGenres;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DramaArticle {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //user
    pub article: String,
    pub show_id: ObjectId,//drama
    pub episode_id: Option<ObjectId>, //episode
    pub genres: Vec<DramaGenres>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}