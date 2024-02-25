use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::enums::{DramaGenres, OriginCountry, Rating};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Show {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub native_title: String,
    pub rating: Rating,
    pub synopsis: String,
    pub country_origin: OriginCountry,
    pub number_episodes: u16,
    pub air_start: DateTime,
    pub air_end: DateTime,
    pub duration: u32,
    pub genres: Vec<DramaGenres>,
    pub cover_image: Option<ObjectId>,
    pub actors: Vec<ObjectId>,
    pub episodes: Vec<ObjectId>,
    pub directors: Vec<ObjectId>,
    pub screenwriters: Vec<ObjectId>,
    pub main_roles: Vec<ObjectId>,
    pub supporting_roles: Vec<ObjectId>,
    pub guest_roles: Vec<ObjectId>,
    pub producers: Vec<ObjectId>,
    pub music_editors: Vec<ObjectId>,
    pub reviews: Vec<ObjectId>,
    pub articles: Vec<ObjectId>,
    pub photos: Vec<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}