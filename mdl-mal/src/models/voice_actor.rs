use bson::DateTime;
use bson::oid::ObjectId;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use crate::models::enums::Gender;
use crate::models::show::Show;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub given_name: String,
    pub family_name: String,
    pub nationality: CountryCode,
    pub gender: Gender,
    pub born: DateTime,
    pub age: u16,
    pub biography: String,
    pub anime: Vec<ObjectId>,
    pub characters: Vec<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}