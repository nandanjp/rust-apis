use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub blogs: Vec<ObjectId>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: String,
    pub blogs: Option<Vec<ObjectId>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub blogs: Option<Vec<ObjectId>>,
}

impl Category {
    pub fn from_create(create: CreateCategory) -> Self {
        Self {
            name: create.name,
            description: create.description,
            blogs: create.blogs.unwrap_or_default(),
            id: ObjectId::new(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub fn update(self, update: UpdateCategory) -> Self {
        Self {
            name: update.name.unwrap_or(self.name),
            description: update.description.unwrap_or(self.description),
            blogs: update.blogs.unwrap_or(self.blogs),
            id: self.id,
            created_at: self.created_at,
            updated_at: DateTime::now(),
        }
    }

    pub fn into_document(self) -> Document {
        doc! {
            "_id": self.id,
            "name": self.name,
            "description": self.description,
            "blogs": self.blogs,
            "createdAt": self.created_at,
            "updatedAt": self.updated_at
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CommonCategoryResponse {
    pub success: bool,
    pub data: Option<Category>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ListCategoryResponse {
    pub success: bool,
    pub data: Option<Vec<Category>>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}
