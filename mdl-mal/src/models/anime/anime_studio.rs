use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Studio {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub established_date: DateTime,
    pub anime: Vec<ObjectId>, //anime
    pub created_at: DateTime,
    pub updated_at: DateTime,
}