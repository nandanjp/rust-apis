use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimeList {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //user
    pub anime: Vec<ObjectId>, //anime
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterList {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //user
    pub characters: Vec<ObjectId>, //characters
    pub created_at: DateTime,
    pub updated_at: DateTime,
}