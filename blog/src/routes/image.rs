use crate::models::image::{CommonImageResponse, CreateImage, Image, ListImageResponse};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;
use bson::oid::ObjectId;
use futures::StreamExt;
use http::StatusCode;
use mongodb::options::{FindOneOptions, FindOptions, InsertOneOptions};
use mongodb::{Client, Collection};
use serde_json::Value;

pub async fn get_images(State(client): State<Client>) -> impl IntoResponse {
    let image_coll: Collection<Image> = client.database("blog_app").collection::<Image>("image");
    let mut images = match image_coll.find(None, FindOptions::default()).await {
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
    let image_coll: Collection<Image> = client.database("blog_app").collection::<Image>("image");
    let image = match image_coll.find_one(None, FindOneOptions::default()).await {
        Ok(img) => img,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(CommonImageResponse {
                    success: false,
                    data: None,
                    error_message: Some(format!(
                        "Failed to retrieve an image with the id = {}: {:#?}",
                        id.clone(),
                        err
                    )),
                }),
            )
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
        None => (
            StatusCode::NOT_FOUND,
            Json(CommonImageResponse {
                success: false,
                data: None,
                error_message: Some(format!(
                    "Failed to retrieve an image with the given id = {id}"
                )),
            }),
        ),
    }
}

pub async fn create_image(
    State(client): State<Client>,
    Json(image): Json<Value>,
) -> impl IntoResponse {
    let cloned = image.clone();
    let image: CreateImage = match serde_json::from_value(image) {
        Ok(image) => image,
        Err(err) => return (
            StatusCode::NOT_FOUND,
            Json(CommonImageResponse {
                success: false,
                data: None,
                error_message: Some(format!("Failed convert the provide json into a create image response (expect a url, description, alt_description and blog_id). Was given {cloned}, error = {:#?}", err))
            })
        )
    };

    let collections = match client
        .database("blog_app")
        .list_collection_names(None)
        .await
    {
        Ok(cols) => cols,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CommonImageResponse {
                    success: false,
                    data: None,
                    error_message: Some(format!(
                        "Failed to retrieve a list of collection names from the database: {:#?}",
                        err
                    )),
                }),
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
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(CommonImageResponse {
                        success: false,
                        data: None,
                        error_message: Some(format!(
                            "Failed to create a new image collection to the database: {:#?}",
                            err
                        )),
                    }),
                )
            }
        }
    }

    let created = match client
        .database("blog_app")
        .collection::<Image>("image")
        .insert_one(Image::from_create(image), InsertOneOptions::default())
        .await
    {
        Ok(v) => v,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CommonImageResponse {
                    success: false,
                    data: None,
                    error_message: Some(format!(
                        "Failed to create a new image in the image collection: {:#?}",
                        err
                    )),
                }),
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
            return (
                StatusCode::NOT_FOUND,
                Json(CommonImageResponse {
                    success: false,
                    data: None,
                    error_message: Some(format!(
                        "Failed to retrieve an image with the given id = {}: {:#?}",
                        created.inserted_id, err
                    )),
                }),
            )
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
        None => (
            StatusCode::NOT_FOUND,
            Json(CommonImageResponse {
                success: false,
                data: None,
                error_message: Some(format!(
                    "Failed to retrieve an image with the given id = {}",
                    created.inserted_id
                )),
            }),
        ),
    }
}
