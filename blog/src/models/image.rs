use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub _id: ObjectId,
    pub url: String,
    pub blog_id: ObjectId,
    pub description: String,
    #[serde(rename = "altDescription")]
    pub alt_description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
