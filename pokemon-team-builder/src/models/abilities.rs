use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::traits::IntoDocument;

#[derive(Serialize, Deserialize, Debug)]
pub enum Abilities {
    AbilityPair(ObjectId, ObjectId),
    AbilityTriple(ObjectId, ObjectId, ObjectId),
    AbilitySingle(ObjectId),
}

impl Abilities {
    pub fn to_string(&self) -> Vec<ObjectId> {
        match self {
            &Abilities::AbilitySingle(id) => vec![id.clone()],
            &Abilities::AbilityPair(first, second) => vec![first.clone(), second.clone()],
            &Abilities::AbilityTriple(first, second, third) => {
                vec![first.clone(), second.clone(), third.clone()]
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    name: String,
    description: String,
    is_hidden: bool,
}

impl Ability {
    fn new(name: String, description: String, is_hidden: bool) -> Self {
        Ability {
            name,
            description,
            is_hidden,
        }
    }
}

impl IntoDocument for Ability {
    fn into_doc(self) -> mongodb::bson::Document {
        doc! {
            "name": self.name,
            "description": self.description,
            "is_hidden": self.is_hidden,
        }
    }
}
