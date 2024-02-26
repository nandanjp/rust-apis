use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DramaPhoto {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub show_id: ObjectId, //drama
    pub created_at: DateTime,
    pub updated_at: DateTime,
}