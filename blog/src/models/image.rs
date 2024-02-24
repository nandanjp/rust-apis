use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub blogs: Vec<ObjectId>,
    pub description: String,
    #[serde(rename = "altDescription")]
    pub alt_description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
#[derive(Clone, Debug, Deserialize)]
pub struct CreateImage {
    pub url: String,
    pub description: String,
    #[serde(rename = "altDescription")]
    pub alt_description: String,
    pub blog_id: Option<ObjectId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateImage {
    pub url: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "altDescription")]
    pub alt_description: Option<String>,
    pub blogs: Option<Vec<ObjectId>>,
}

impl Image {
    pub fn from_create(create: CreateImage) -> Self {
        Self {
            url: create.url,
            description: create.description,
            alt_description: create.alt_description,
            blogs: create.blog_id.map_or(Vec::new(), |id| vec![id]),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            id: ObjectId::new(),
        }
    }

    pub fn update(self, update: UpdateImage) -> Self {
        Self {
            url: update.url.unwrap_or(self.url),
            description: update.description.unwrap_or(self.description),
            alt_description: update.alt_description.unwrap_or(self.alt_description),
            blogs: update.blogs.unwrap_or(self.blogs),
            created_at: self.updated_at,
            updated_at: DateTime::now(),
            id: self.id,
        }
    }

    pub fn into_document(self) -> Document {
        doc! {
            "_id": self.id,
            "url": self.url,
            "description": self.description,
            "altDescription": self.alt_description,
            "blogs": self.blogs,
            "createdAt": self.created_at,
            "updatedAt": self.updated_at
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ListImageResponse {
    pub success: bool,
    pub data: Option<Vec<Image>>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CommonImageResponse {
    pub success: bool,
    pub data: Option<Image>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}
