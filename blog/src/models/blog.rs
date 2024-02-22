use crate::models::common::Order;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blog {
    pub _id: ObjectId,
    pub title: String,
    pub username: String,
    pub markdown: String,
    #[serde(rename = "frontImage")]
    pub front_image: Option<ObjectId>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}

impl Blog {
    pub fn from_create(create: CreateBlog) -> Self {
        Blog {
            title: create.title,
            username: create.username,
            markdown: create.markdown,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            _id: ObjectId::new(),
            front_image: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBlog {
    pub title: String,
    pub username: String,
    pub markdown: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListBlogResponse {
    pub success: bool,
    pub data: Option<Vec<Blog>>,
    pub error_message: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonBlogResponse {
    pub success: bool,
    pub data: Option<Blog>,
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
