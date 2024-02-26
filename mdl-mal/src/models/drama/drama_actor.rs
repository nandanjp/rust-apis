use bson::DateTime;
use bson::oid::ObjectId;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use crate::utils::enums::actor_type::ActorType;
use crate::utils::enums::gender::Gender;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DramaActor {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub first_name: String,
    pub family_name: String,
    pub native_name: String,
    pub nationality: CountryCode,
    pub gender: Gender,
    pub born: DateTime,
    pub age: u16,
    pub biography: String,
    pub kind: ActorType,
    pub produced: Vec<ObjectId>, //dramas
    pub dramas: Vec<ObjectId>, //dramas
    pub movies: Vec<ObjectId>, //dramas
    pub articles: Vec<ObjectId>, //articles
    pub created_at: DateTime,
    pub updated_at: DateTime,
}