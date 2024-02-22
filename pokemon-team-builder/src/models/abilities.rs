use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Abilities {
    AbilityPair(ObjectId, ObjectId),
    AbilityTriple(ObjectId, ObjectId, ObjectId),
    AbilitySingle(ObjectId),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    name: String,
    description: String,
    is_hidden: bool,
}

impl Ability {}
