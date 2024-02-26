use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DramaList {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //user
    pub shows: Vec<ObjectId>, //dramas
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActorList {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //users
    pub actors: Vec<ObjectId>, //actors
    pub created_at: DateTime,
    pub updated_at: DateTime,
}