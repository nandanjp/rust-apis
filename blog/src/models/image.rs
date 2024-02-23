use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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

impl Image {
    pub fn from_create(create: CreateImage) -> Self {
        Self {
            url: create.url,
            description: create.description,
            alt_description: create.alt_description,
            blog_id: create.blog_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            _id: ObjectId::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateImage {
    pub url: String,
    pub description: String,
    #[serde(rename = "altDescription")]
    pub alt_description: String,
    pub blog_id: ObjectId,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListImageResponse {
    pub success: bool,
    pub data: Option<Vec<Image>>,
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonImageResponse {
    pub success: bool,
    pub data: Option<Image>,
    pub error_message: Option<String>,
}
