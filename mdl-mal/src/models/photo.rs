use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub show_id: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}