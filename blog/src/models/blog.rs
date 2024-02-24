use crate::models::common::Order;
use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blog {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub username: String,
    pub markdown: String,
    #[serde(rename = "frontImage")]
    pub front_image: Option<ObjectId>,
    pub images: Vec<ObjectId>,
    pub categories: Vec<ObjectId>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBlog {
    pub title: String,
    pub username: String,
    pub markdown: String,
    #[serde(rename = "frontImage")]
    pub front_image: Option<ObjectId>,
    pub images: Option<Vec<ObjectId>>,
    pub categories: Option<Vec<ObjectId>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateBlog {
    pub title: Option<String>,
    pub username: Option<String>,
    pub markdown: Option<String>,
    #[serde(rename = "frontImage")]
    pub front_image: Option<ObjectId>,
    pub images: Option<Vec<ObjectId>>,
    pub categories: Option<Vec<ObjectId>>,
}

impl Blog {
    pub fn from_create(create: CreateBlog) -> Self {
        Self {
            title: create.title,
            username: create.username,
            markdown: create.markdown,
            id: ObjectId::new(),
            front_image: create.front_image.map(|image| image),
            images: create.images.unwrap_or_default(),
            categories: create.categories.unwrap_or_default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub fn update(self, update: UpdateBlog) -> Self {
        Self {
            title: update.title.unwrap_or(self.title),
            username: update.username.unwrap_or(self.username),
            markdown: update.markdown.unwrap_or(self.markdown),
            front_image: update.front_image.or(self.front_image),
            images: update.images.unwrap_or(self.images),
            categories: update.categories.unwrap_or(self.categories),
            id: self.id,
            created_at: self.created_at,
            updated_at: DateTime::now(),
        }
    }

    pub fn into_document(self) -> Document {
        doc! {
            "_id": self.id,
            "title": self.title,
            "username": self.username,
            "markdown": self.markdown,
            "frontImage": self.front_image,
            "images": self.images,
            "categories": self.categories,
            "createdAt": self.created_at,
            "updatedAt": self.updated_at
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ListBlogResponse {
    pub success: bool,
    pub data: Option<Vec<Blog>>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CommonBlogResponse {
    pub success: bool,
    pub data: Option<Blog>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_order")]
    pub order: Order,
}
fn default_page() -> i32 {
    1
}
fn default_per_page() -> i32 {
    20
}
fn default_sort_by() -> String {
    String::from("title")
}
fn default_order() -> Order {
    Order::Asc
}

impl Pagination {
    pub fn validate(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("Page must be greater than or equal to 1.".into());
        }

        if self.per_page < 1 {
            return Err("Rows per page must be greater than or equal to 1.".into());
        } else if self.per_page > 100 {
            return Err("Rows per page must be less than or equal to 100.".into());
        }

        if !([
            "_id".to_string(),
            "title".to_string(),
            "username".to_string(),
        ]
        .contains(&self.sort_by))
        {
            return Err("Invalid value passed for sort_by query parameter. Must be one of: _id, title or username.".into());
        }

        Ok(())
    }
}
