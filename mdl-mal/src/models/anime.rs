use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::enums::{AnimeGenres, Rating};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anime {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub rating: Rating,
    pub synopsis: String,
    pub number_episodes: u16,
    pub air_start: DateTime,
    pub air_end: DateTime,
    pub duration: u16,
    pub genres: AnimeGenres,
    pub characters: String,
    pub cover_image: Option<ObjectId>,
    pub studios: Vec<ObjectId>,
    pub episodes: Vec<ObjectId>,
    pub producers: Vec<ObjectId>,
    pub voice_actors: Vec<ObjectId>,
    pub main_roles: Vec<ObjectId>,
    pub reviews: Vec<ObjectId>,
    pub articles: Vec<ObjectId>,
    pub photos: Vec<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}