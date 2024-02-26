use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DramaActorArticle {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId, //user
    pub article: String,
    pub actor_id: ObjectId, //actor
    pub created_at: DateTime,
    pub updated_at: DateTime,
}