use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::enums::rating::Rating;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterReview {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub review: String,
    pub overall_score: Rating,
    pub user_id: ObjectId, //user
    pub character_id: ObjectId, //character
    pub created_at: DateTime,
    pub updated_at: DateTime,
}