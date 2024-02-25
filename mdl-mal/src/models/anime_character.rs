use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::enums::{DramaGenres, Gender, OriginCountry, UserType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub voice_actors: Vec<ObjectId>,
    pub anime: ObjectId,
    pub reviews: Vec<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}