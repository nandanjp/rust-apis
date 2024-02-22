use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::traits::IntoDocument;

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    name: String,
    description: String,
    damage: Option<u8>,
}

impl Move {
    fn new(name: String, description: String, damage: Option<u8>) -> Self {
        Move {
            name,
            description,
            damage,
        }
    }
}

impl IntoDocument for Move {
    fn into_doc(self) -> mongodb::bson::Document {
        doc! {
            "name": self.name,
            "description": self.description,
            "damage": self.damage.unwrap_or(0) as u32,
        }
    }
}
