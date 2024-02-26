use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::enums::gender::Gender;
use crate::utils::enums::user_type::UserType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub gender: Gender,
    pub location: String,
    pub date_of_birth: DateTime,
    pub biography: String,
    pub kind: UserType,
    pub lists: Vec<ObjectId>,
    pub articles: Vec<ObjectId>,
    pub reviews: Vec<ObjectId>,
    pub currently_watching: Vec<ObjectId>,
    pub completed: Vec<ObjectId>,
    pub plan_to_watch: Vec<ObjectId>,
    pub dropped: Vec<ObjectId>,
    pub currently_watching_anime: Vec<ObjectId>,
    pub completed_anime: Vec<ObjectId>,
    pub plan_to_watch_anime: Vec<ObjectId>,
    pub dropped_anime: Vec<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}