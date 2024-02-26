use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::enums::anime_genres::AnimeGenres;
use crate::utils::enums::rating::Rating;

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
    pub characters: Vec<ObjectId>, //characters
    pub cover_image_id: Option<ObjectId>, //photo
    pub studios: Vec<ObjectId>, //studios
    pub episodes: Vec<ObjectId>, //episodes
    pub producers: Vec<ObjectId>, //actors
    pub voice_actors: Vec<ObjectId>, //actors
    pub main_roles: Vec<ObjectId>, //actors
    pub reviews: Vec<ObjectId>, //reviews
    pub articles: Vec<ObjectId>, //articles
    pub photos: Vec<ObjectId>, //photos
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Anime {

}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateAnime {
    pub title: String,
    pub rating: Rating,
    pub synopsis: String,
    pub number_episodes: u16,
    pub air_start: DateTime,
    pub air_end: DateTime,
    pub duration: u16,
    pub genres: Vec<AnimeGenres>,
    pub characters: Vec<ObjectId>, //characters
    pub cover_image_id: Option<ObjectId>, //photo
    pub studios: Vec<ObjectId>, //studios
    pub producers: Vec<ObjectId>, //actors
    pub voice_actors: Vec<ObjectId>, //actors
    pub main_roles: Vec<ObjectId>, //actors
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateAnime {
    pub title: Option<String>,
    pub rating: Option<Rating>,
    pub synopsis: Option<String>,
    pub number_episodes: Option<u16>,
    pub air_start: Option<DateTime>,
    pub air_end: Option<DateTime>,
    pub duration: Option<u16>,
    pub genres: Option<AnimeGenres>,
    pub characters: Option<Vec<ObjectId>>, //characters
    pub cover_image_id: Option<ObjectId>, //photo
    pub studios: Option<Vec<ObjectId>>, //studios
    pub episodes: Option<Vec<ObjectId>>, //episodes
    pub producers: Option<Vec<ObjectId>>, //actors
    pub voice_actors: Option<Vec<ObjectId>>, //actors
    pub main_roles: Option<Vec<ObjectId>>, //actors
}
impl CreateAnime {

}