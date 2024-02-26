use bson::DateTime;
use bson::oid::ObjectId;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use crate::utils::enums::drama_genres::DramaGenres;
use crate::utils::enums::rating::Rating;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Show {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub native_title: String,
    pub rating: Rating,
    pub synopsis: String,
    pub country_origin: CountryCode,
    pub number_episodes: u16,
    pub air_start: DateTime,
    pub air_end: DateTime,
    pub duration: u32,
    pub genres: Vec<DramaGenres>,
    pub cover_image: Option<ObjectId>, //drama_photo
    pub actors: Vec<ObjectId>, //drama_actors
    pub episodes: Vec<ObjectId>, //drama_episodes
    pub directors: Vec<ObjectId>, //drama_actors
    pub screenwriters: Vec<ObjectId>, //drama_actors
    pub main_roles: Vec<ObjectId>, //drama_actors
    pub supporting_roles: Vec<ObjectId>, //drama_actors
    pub guest_roles: Vec<ObjectId>, //drama_actors
    pub producers: Vec<ObjectId>, //drama_actors
    pub music_editors: Vec<ObjectId>, //drama_actors
    pub reviews: Vec<ObjectId>, //drama_reviews
    pub articles: Vec<ObjectId>, //drama_articles
    pub photos: Vec<ObjectId>, //drama_photos
    pub created_at: DateTime,
    pub updated_at: DateTime,
}