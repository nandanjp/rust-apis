use bson::DateTime;
use bson::oid::ObjectId;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use crate::utils::enums::gender::Gender;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceActor {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub given_name: String,
    pub family_name: String,
    pub nationality: CountryCode,
    pub gender: Gender,
    pub born: DateTime,
    pub age: u16,
    pub biography: String,
    pub anime: Vec<ObjectId>, //anime
    pub characters: Vec<ObjectId>, //characters
    pub created_at: DateTime,
    pub updated_at: DateTime,
}