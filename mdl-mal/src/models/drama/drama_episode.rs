use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::enums::rating::Rating;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DramaEpisode {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub episode_number: u16,
    pub rating: Rating,
    pub release_date: DateTime,
    pub show_id: ObjectId, //drama
    pub created_at: DateTime,
    pub updated_at: DateTime,
}