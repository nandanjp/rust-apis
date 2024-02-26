use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimePhoto {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub anime_id: ObjectId, //anime
    pub created_at: DateTime,
    pub updated_at: DateTime,
}