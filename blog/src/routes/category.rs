use crate::models::category::{Category, CommonCategoryResponse, ListCategoryResponse};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;
use bson::oid::ObjectId;
use futures::StreamExt;
use http::StatusCode;
use mongodb::options::{
    CreateCollectionOptions, FindOneAndDeleteOptions, FindOneAndReplaceOptions, FindOneOptions,
    FindOptions, InsertOneOptions,
};
use mongodb::{Client, Collection};
use serde_json::Value;

fn get_collection(client: &Client) -> Collection<Category> {
    client
        .database("blog_app")
        .collection::<Category>("category")
}

fn failed_category_response(message: String) -> (StatusCode, Json<CommonCategoryResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(CommonCategoryResponse {
            success: false,
            data: None,
            error_message: Some(message),
        }),
    )
}

fn failed_category_response_general(
    status: StatusCode,
    message: String,
) -> (StatusCode, Json<CommonCategoryResponse>) {
    (
        status,
        Json(CommonCategoryResponse {
            success: false,
            data: None,
            error_message: Some(message),
        }),
    )
}

pub async fn get_categories(State(client): State<Client>) -> impl IntoResponse {
    let mut category_cursor = match get_collection(&client).find(None, FindOptions::default()).await {
        Ok(categories) => categories,
        Err(err) => return (
            StatusCode::INTERNAL_SERVER_ERROR, Json(ListCategoryResponse {
                success: false,
                data: None,
                error_message: Some(format!("Failed to get all categories from the collection due to the following error: {:#?}", err))
            })
        )
    };
    let mut categories: Vec<Category> = Vec::new();
    while let Some(category) = category_cursor.next().await {
        categories.push(
            category.expect("Failed to retrieve a specific category from the category collection"),
        )
    }
    (
        StatusCode::OK,
        Json(ListCategoryResponse {
            success: true,
            data: Some(categories),
            error_message: None,
        }),
    )
}

pub async fn get_category_by_id(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
) -> impl IntoResponse {
    let category = match get_collection(&client)
        .find_one(doc! { "_id": id }, FindOneOptions::default())
        .await
    {
        Ok(category) => match category {
            Some(category) => category,
            None => {
                return failed_category_response(format!(
                    "Could not find a category with the provided id = {id}"
                ))
            }
        },
        Err(err) => {
            return failed_category_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Could not find a category with the provided id = {id}, err: {:#?}",
                    err
                ),
            )
        }
    };
    (
        StatusCode::FOUND,
        Json(CommonCategoryResponse {
            success: true,
            data: Some(category),
            error_message: None,
        }),
    )
}

pub async fn create_category(
    State(client): State<Client>,
    Json(create): Json<Value>,
) -> impl IntoResponse {
    let cloned = create.clone();
    let create = match serde_json::from_value(create) {
        Ok(create) => Category::from_create(create),
        Err(err) => return failed_category_response_general(StatusCode::BAD_REQUEST, format!("Failed to parse the provided json payload into create category payload. Provided {cloned}, and produced the error: {:#?}", err))
    };

    let collections = match client
        .database("blog_app")
        .list_collection_names(None)
        .await
    {
        Ok(collections) => collections,
        Err(err) => {
            return failed_category_response_general(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to get a list of collection names from the database: {:#?}",
                    err
                ),
            )
        }
    };

    if collections
        .into_iter()
        .find(|name| *name == "category")
        .is_none()
    {
        match client
            .database("blog_app")
            .create_collection("category", CreateCollectionOptions::default())
            .await
        {
            Ok(_) => tracing::debug!("Successfully created the category collection"),
            Err(err) => {
                return failed_category_response_general(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Failed to create a new category collection in the database: {:#?}",
                        err
                    ),
                )
            }
        }
    }

    let created = match get_collection(&client)
        .insert_one(create, InsertOneOptions::default())
        .await
    {
        Ok(inserted) => match get_collection(&client)
            .find_one(
                doc! { "_id": inserted.inserted_id.clone() },
                FindOneOptions::default(),
            )
            .await
        {
            Ok(created) => match created {
                Some(created) => created,
                None => {
                    return failed_category_response(format!(
                        "Failed to create and retrieve a new category with the id of {}",
                        inserted.inserted_id
                    ))
                }
            },
            Err(err) => {
                return failed_category_response_general(
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Failed to create and retrieve a new category with the id of {}: {:#?}",
                        inserted.inserted_id, err
                    ),
                )
            }
        },
        Err(err) => {
            return failed_category_response_general(
                StatusCode::BAD_REQUEST,
                format!("Failed to create a new category: {:#?}", err),
            )
        }
    };

    (
        StatusCode::CREATED,
        Json(CommonCategoryResponse {
            success: true,
            data: Some(created),
            error_message: None,
        }),
    )
}

pub async fn update_category(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
    Json(update): Json<Value>,
) -> impl IntoResponse {
    let cloned = update.clone();
    let update = match serde_json::from_value(update) {
        Ok(update) => match get_collection(&client).find_one(doc! { "_id": id }, FindOneOptions::default()).await {
            Ok(category) => match category {
                Some(category) => category.update(update),
                None => return failed_category_response(format!(
                    "Could not find a category with the id = {id}"
                ))
            },
            Err(err) => return failed_category_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Failed to find a category with the id = {id} from the categories collection: {:#?}", err
                ),
            )
        },
        Err(err) => return failed_category_response_general(
            StatusCode::BAD_REQUEST,
            format!(
                "Failed to parse the given json payload into an update category payload. Was given the payload = {cloned} and received the error: {:#?}", err
            ),
        )
    };

    let updated = match get_collection(&client)
        .find_one_and_replace(
            doc! { "_id": id },
            update,
            FindOneAndReplaceOptions::default(),
        )
        .await
    {
        Ok(updated) => match updated {
            Some(updated) => updated,
            None => {
                return failed_category_response(format!(
                    "Could not find a category with the id = {id}"
                ))
            }
        },
        Err(err) => {
            return failed_category_response_general(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to update the category with the id = {id}: {:#?}",
                    err
                ),
            )
        }
    };

    (
        StatusCode::OK,
        Json(CommonCategoryResponse {
            success: true,
            data: Some(updated),
            error_message: None,
        }),
    )
}

pub async fn delete_category(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
) -> impl IntoResponse {
    let deleted = match get_collection(&client)
        .find_one_and_delete(doc! { "_id": id }, FindOneAndDeleteOptions::default())
        .await
    {
        Ok(deleted) => match deleted {
            Some(deleted) => deleted,
            None => {
                return failed_category_response(format!(
                    "Could not find a category with the id = {id}"
                ))
            }
        },
        Err(err) => {
            return failed_category_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Failed to delete the category with the id = {id}: {:#?}",
                    err
                ),
            )
        }
    };
    (
        StatusCode::OK,
        Json(CommonCategoryResponse {
            success: true,
            data: Some(deleted),
            error_message: None,
        }),
    )
}
