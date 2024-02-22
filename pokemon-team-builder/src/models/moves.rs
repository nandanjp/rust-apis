use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    name: String,
    description: String,
    damage: Option<u8>,
}

impl Move {}
