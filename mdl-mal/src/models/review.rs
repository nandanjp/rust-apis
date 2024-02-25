use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::enums::Rating;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub has_spoiler: bool,
    pub episodes_seen: u16,
    pub review: String,
    pub overall_score: Rating,
    pub story_score: Rating,
    pub acting_score: Rating,
    pub music_score: Rating,
    pub rewatch_score: Rating,
    pub author_id: ObjectId,
    pub show_id: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}