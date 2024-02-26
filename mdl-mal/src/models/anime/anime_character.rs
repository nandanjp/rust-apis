use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub voice_actors: Vec<ObjectId>, //voice_actors
    pub anime: ObjectId, //anime
    pub reviews: Vec<ObjectId>, //reviews
    pub created_at: DateTime,
    pub updated_at: DateTime,
}