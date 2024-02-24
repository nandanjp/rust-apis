use crate::models::blog::{Blog, CommonBlogResponse, CreateBlog, ListBlogResponse, Pagination};
use crate::models::common::Order;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;
use bson::oid::ObjectId;
use futures::StreamExt;
use http::StatusCode;
use mongodb::options::{
    FindOneAndDeleteOptions, FindOneAndReplaceOptions, FindOneOptions, FindOptions,
    InsertOneOptions,
};
use mongodb::{Client, Collection};
use serde_json::Value;

fn get_collection(client: &Client) -> Collection<Blog> {
    client.database("blog_app").collection::<Blog>("blog")
}

fn failed_response(message: String) -> (StatusCode, Json<CommonBlogResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(CommonBlogResponse {
            success: false,
            data: None,
            error_message: Some(message),
        }),
    )
}

fn failed_response_general(
    status: StatusCode,
    message: String,
) -> (StatusCode, Json<CommonBlogResponse>) {
    (
        status,
        Json(CommonBlogResponse {
            success: false,
            data: None,
            error_message: Some(message),
        }),
    )
}

pub async fn get_blogs(
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

    let mut options = FindOptions::default();
    options.sort = Some(doc! { &pagination.sort_by: match pagination.order {
        Order::Asc => 1,
        Order::Desc => -1
    } });
    options.limit = Some(pagination.per_page as i64);
    options.skip = Some((pagination.page as u64 - 1) * pagination.per_page as u64);

    let mut blog_cursor = get_collection(&client)
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

pub async fn get_blog_by_id(State(client): State<Client>, id: Path<ObjectId>) -> impl IntoResponse {
    let blog = get_collection(&client)
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
            None => failed_response(format!("No blog exists for given id: {}", id.0.clone())),
        },
        Err(err) => failed_response(format!("Couldn't find any user due to {:#?}", err)),
    }
}

pub async fn create_blog(
    State(client): State<Client>,
    Json(blog): Json<Value>,
) -> impl IntoResponse {
    let cloned = blog.clone();
    let blog: CreateBlog = match serde_json::from_value(blog) {
        Ok(Some(blog)) => blog,
        Ok(None) => {
            return failed_response(format!(
                "Could not create a blog from the body provided: {}",
                cloned
            ))
        }
        Err(err) => {
            return failed_response(format!(
                "Could not create a blog from the body provided: {}, err = {}",
                cloned, err
            ))
        }
    };

    let collections = match client
        .database("blog_app")
        .list_collection_names(None)
        .await
    {
        Ok(names) => names,
        Err(err) => {
            return failed_response(format!(
                "Failed to get a list of collection names in the database: {:#?}",
                err
            ))
        }
    };

    if collections
        .into_iter()
        .find(|name| *name == "blog")
        .is_none()
    {
        match client
            .database("blog_app")
            .create_collection("blog", None)
            .await
        {
            Ok(_) => tracing::debug!("Successfully created the blog collection"),
            Err(err) => {
                return failed_response(format!(
                    "Failed to create a blog collection in the blog_app database: {:#?}",
                    err
                ))
            }
        }
    }

    match get_collection(&client)
        .insert_one(Blog::from_create(blog), InsertOneOptions::default())
        .await
    {
        Ok(blog) => {
            match get_collection(&client)
                .find_one(
                    doc! {"_id": blog.inserted_id.clone()},
                    FindOneOptions::default(),
                )
                .await
            {
                Ok(Some(blog)) => (
                    StatusCode::CREATED,
                    Json(CommonBlogResponse {
                        success: true,
                        data: Some(blog),
                        error_message: None,
                    }),
                ),
                Ok(None) => {
                    failed_response(format!("No blog exists for given id: {}", blog.inserted_id))
                }
                Err(err) => failed_response(format!("Could not find any blog due to {:#?}", err)),
            }
        }
        Err(err) => failed_response(format!("Could not find any blog due to {:#?}", err)),
    }
}

pub async fn update_blog(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
    Json(update): Json<Value>,
) -> impl IntoResponse {
    let blog = match get_collection(&client)
        .find_one(doc! {"_id": id}, FindOneOptions::default())
        .await
    {
        Ok(blog) => match blog {
            Some(blog) => blog,
            None => return failed_response(format!("Could not find any blog with the id = {id}.")),
        },
        Err(err) => {
            return failed_response(format!(
                "Could not find any blog with the id = {id}: err = {:#?}",
                err
            ))
        }
    };

    let cloned = update.clone();
    let update = match serde_json::from_value(update) {
        Ok(update) => blog.update(update),
        Err(err) => return failed_response_general(StatusCode::BAD_REQUEST, format!("Could not parse the given json object into an update_blog object, update = {cloned}: err = {:#?}", err))
    };

    let updated = match get_collection(&client)
        .find_one_and_replace(
            doc! {"_id": update.id},
            update,
            FindOneAndReplaceOptions::default(),
        )
        .await
    {
        Ok(updated) => match updated {
            Some(blog) => blog,
            None => {
                return failed_response_general(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to update the blog with the given id = {id}."),
                )
            }
        },
        Err(err) => {
            return failed_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Failed to update the blog with the given id = {id}: err = {:#?}",
                    err
                ),
            )
        }
    };

    (
        StatusCode::CREATED,
        Json(CommonBlogResponse {
            success: true,
            data: Some(updated),
            error_message: None,
        }),
    )
}

pub async fn delete_blog(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
) -> impl IntoResponse {
    let deleted = match get_collection(&client).find_one_and_delete(doc! { "_id": id }, FindOneAndDeleteOptions::default()).await {
        Ok(deleted) => match deleted {
            Some(deleted) => deleted,
            None => return failed_response_general(StatusCode::BAD_REQUEST,format!("Failed to delete the blog with the given id = {id}."))
        },
        Err(err) => return failed_response_general(StatusCode::BAD_REQUEST, format!("Failed to delete the blog with the given id = {id} due to the following error: {:#?}", err))
    };
    (
        StatusCode::OK,
        Json(CommonBlogResponse {
            success: true,
            data: Some(deleted),
            error_message: None,
        }),
    )
}
