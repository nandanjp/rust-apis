use crate::models::image::{CommonImageResponse, CreateImage, Image, ListImageResponse};
use axum::extract::{Path, State};
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

fn get_collection(client: &Client) -> Collection<Image> {
    client.database("blog_app").collection::<Image>("image")
}

fn failed_common_response(message: String) -> (StatusCode, Json<CommonImageResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(CommonImageResponse {
            success: false,
            data: None,
            error_message: Some(message),
        }),
    )
}
fn failed_common_response_general(
    status: StatusCode,
    message: String,
) -> (StatusCode, Json<CommonImageResponse>) {
    (
        status,
        Json(CommonImageResponse {
            success: false,
            data: None,
            error_message: Some(message),
        }),
    )
}

pub async fn get_images(State(client): State<Client>) -> impl IntoResponse {
    let mut images = match get_collection(&client)
        .find(None, FindOptions::default())
        .await
    {
        Ok(images) => images,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ListImageResponse {
                    success: false,
                    data: None,
                    error_message: Some(format!(
                        "Failed to retrieve all images in the collection: {:#?}",
                        err
                    )),
                }),
            )
        }
    };

    let mut imgs: Vec<Image> = Vec::new();
    while let Some(img) = images.next().await {
        imgs.push(img.expect("could not load image information"))
    }
    (
        StatusCode::OK,
        Json(ListImageResponse {
            success: true,
            data: Some(imgs),
            error_message: None,
        }),
    )
}

pub async fn get_image_by_id(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
) -> impl IntoResponse {
    let image = match get_collection(&client)
        .find_one(None, FindOneOptions::default())
        .await
    {
        Ok(img) => img,
        Err(err) => {
            return failed_common_response(format!(
                "Failed to retrieve an image with the id = {}: {:#?}",
                id.clone(),
                err
            ))
        }
    };
    match image {
        Some(image) => (
            StatusCode::FOUND,
            Json(CommonImageResponse {
                success: true,
                data: Some(image),
                error_message: None,
            }),
        ),
        None => failed_common_response(format!(
            "Failed to retrieve an image with the given id = {id}"
        )),
    }
}

pub async fn create_image(
    State(client): State<Client>,
    Json(image): Json<Value>,
) -> impl IntoResponse {
    let cloned = image.clone();
    let image: CreateImage = match serde_json::from_value(image) {
        Ok(image) => image,
        Err(err) =>
        return failed_common_response(format!("Failed convert the provide json into a create image response (expect a url, description, alt_description and blog_id). Was given {cloned}, error = {:#?}", err))
    };

    let collections = match client
        .database("blog_app")
        .list_collection_names(None)
        .await
    {
        Ok(cols) => cols,
        Err(err) => {
            return failed_common_response_general(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to retrieve a list of collection names from the database: {:#?}",
                    err
                ),
            )
        }
    };

    if collections
        .into_iter()
        .find(|name| *name == "image")
        .is_none()
    {
        match client
            .database("blog_app")
            .create_collection("image", None)
            .await
        {
            Ok(_) => tracing::debug!("Successfully created the blog collection"),
            Err(err) => {
                return failed_common_response_general(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Failed to create a new image collection to the database: {:#?}",
                        err
                    ),
                )
            }
        }
    }

    let created = match get_collection(&client)
        .insert_one(Image::from_create(image), InsertOneOptions::default())
        .await
    {
        Ok(v) => v,
        Err(err) => {
            return failed_common_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Failed to create a new image in the image collection: {:#?}",
                    err
                ),
            )
        }
    };
    let image = match client
        .database("blog_app")
        .collection::<Image>("image")
        .find_one(
            doc! { "_id": created.inserted_id.clone() },
            FindOneOptions::default(),
        )
        .await
    {
        Ok(image) => image,
        Err(err) => {
            return failed_common_response(format!(
                "Failed to retrieve an image with the given id = {}: {:#?}",
                created.inserted_id, err
            ))
        }
    };
    match image {
        Some(image) => (
            StatusCode::CREATED,
            Json(CommonImageResponse {
                success: true,
                data: Some(image),
                error_message: None,
            }),
        ),
        None => failed_common_response(format!(
            "Failed to retrieve an image with the given id = {}",
            created.inserted_id
        )),
    }
}

pub async fn update_image(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
    Json(update): Json<Value>,
) -> impl IntoResponse {
    let image = match get_collection(&client)
        .find_one(doc! { "_id": id }, FindOneOptions::default())
        .await
    {
        Ok(image) => match image {
            Some(image) => image,
            None => {
                return failed_common_response(format!(
                    "Failed to find an image with the id = {id}."
                ))
            }
        },
        Err(err) => {
            return failed_common_response(format!(
                "Failed to find an image with the id = {id} due to the error = {:#?}",
                err
            ))
        }
    };

    let cloned = update.clone();
    let update = match serde_json::from_value(update) {
        Ok(update) => image.update(update),
        Err(err) => return failed_common_response_general(StatusCode::BAD_REQUEST, format!("Failed to create an update image body from the given JSON payload, update = {cloned}: err = {:#?}", err))
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
                return failed_common_response_general(
                    StatusCode::BAD_REQUEST,
                    format!("Could not find an image with the id = {id}."),
                )
            }
        },
        Err(err) => {
            return failed_common_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Failed to update the given image, id = {id} due to the following error: {:#?}",
                    err
                ),
            )
        }
    };
    (
        StatusCode::OK,
        Json(CommonImageResponse {
            success: true,
            data: Some(updated),
            error_message: None,
        }),
    )
}

pub async fn delete_image(
    State(client): State<Client>,
    Path(id): Path<ObjectId>,
) -> impl IntoResponse {
    let deleted = match get_collection(&client)
        .find_one_and_delete(doc! {"_id": id}, FindOneAndDeleteOptions::default())
        .await
    {
        Ok(deleted) => match deleted {
            Some(deleted) => deleted,
            None => {
                return failed_common_response(format!(
                    "Could not find an image with the id = {id}."
                ))
            }
        },
        Err(err) => {
            return failed_common_response_general(
                StatusCode::BAD_REQUEST,
                format!(
                    "Failed to delete the given image, id = {id} due to the following error: {:#?}",
                    err
                ),
            )
        }
    };
    (
        StatusCode::OK,
        Json(CommonImageResponse {
            success: true,
            data: Some(deleted),
            error_message: None,
        }),
    )
}
