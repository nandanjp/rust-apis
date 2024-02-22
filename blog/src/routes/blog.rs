use crate::models::blog::{Blog, CommonBlogResponse, CreateBlog, ListBlogResponse, Pagination};
use crate::models::common::Order;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;
use futures::StreamExt;
use http::StatusCode;
use mongodb::options::{FindOneOptions, FindOptions};
use mongodb::{Client, Collection};
use serde_json::Value;

pub async fn get_all_blogs(
    State(client): State<Client>,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    if let Err(message) = pagination.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ListBlogResponse {
                success: false,
                data: None,
                error_message: Some(format!("pagination: {:#?}, {message}", pagination.0)),
            }),
        );
    }
    let blog_coll: Collection<Blog> = client.database("blog_app").collection::<Blog>("blog");
    let mut options = FindOptions::default();
    options.sort = Some(doc! { &pagination.sort_by: match pagination.order {
        Order::Asc => 1,
        Order::Desc => -1
    } });
    options.limit = Some(pagination.per_page as i64);
    options.skip = Some((pagination.page as u64 - 1) * pagination.per_page as u64);
    let mut blog_cursor = blog_coll
        .find(None, options)
        .await
        .expect("could not retrieve blog data");
    let mut blogs: Vec<Blog> = Vec::new();
    while let Some(blog) = blog_cursor.next().await {
        blogs.push(blog.expect("could not load blog information"))
    }
    (
        StatusCode::OK,
        Json(ListBlogResponse {
            success: true,
            data: Some(blogs),
            error_message: None,
        }),
    )
}

pub async fn blog_by_id(State(client): State<Client>, id: Path<String>) -> impl IntoResponse {
    let blog_coll: Collection<Blog> = client.database("blog_app").collection::<Blog>("blog");
    let blog = blog_coll
        .find_one(doc! {"_id": id.0.clone()}, FindOneOptions::default())
        .await;
    match blog {
        Ok(blog) => match blog {
            Some(blog) => (
                StatusCode::FOUND,
                Json(CommonBlogResponse {
                    success: true,
                    data: Some(blog),
                    error_message: None,
                }),
            ),
            None => (
                StatusCode::NOT_FOUND,
                Json(CommonBlogResponse {
                    success: false,
                    data: None,
                    error_message: Some(format!("No blog exists for given id: {}", id.0.clone())),
                }),
            ),
        },
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(CommonBlogResponse {
                success: false,
                data: None,
                error_message: Some(format!("Couldn't find any user due to {:#?}", err)),
            }),
        ),
    }
}

pub async fn create_blog(State(client): State<Client>, Json(blog): Json<Value>) -> impl IntoResponse {
    let cloned = blog.clone();
    let blog: CreateBlog = match serde_json::from_value(blog) {
        Ok(Some(blog)) => blog,
        Ok(None) => return (
            StatusCode::NOT_FOUND,
            Json(CommonBlogResponse {
                success: false,
                data: None,
                error_message: Some(format!("Could not create a blog from the body provided: {}", cloned)),
            }),
        ),
        Err(err) => return (
            StatusCode::NOT_FOUND,
            Json(CommonBlogResponse {
                success: false,
                data: None,
                error_message: Some(format!("Could not create a blog from the body provided: {}, err = {}", cloned, err)),
            }),
        )
    };

    (
        StatusCode::CREATED,
        Json(CommonBlogResponse {
            success: true,
            data: Some(Blog::from_create(blog)),
            error_message: None,
        }),
    )
}