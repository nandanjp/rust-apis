use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use super::{
    abilities::Abilities,
    enums::{Gender, OriginGame, Stat, Type},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pokedex_id: u16,
    name: String,
    alternate_names: Vec<String>,
    origin_gen: OriginGame,
    typ: Type,
    stat: Stat,
    gender: Gender,
    level: Option<u8>,
    abilities: Abilities,
    moves: Vec<ObjectId>,
}

impl Pokemon {
    fn new(
        pokedex_id: u16,
        name: String,
        alternate_names: Vec<String>,
        origin_gen: OriginGame,
        typ: Type,
        stat: Stat,
        gender: Gender,
        level: Option<u8>,
        abilities: Abilities,
        moves: Vec<ObjectId>,
    ) -> Self {
        Pokemon {
            pokedex_id,
            name,
            alternate_names,
            origin_gen,
            typ,
            stat,
            gender,
            level,
            abilities,
            moves,
        }
    }

    fn into_doc(self) -> Document {
        doc! {
            "pokedex_id": self.pokedex_id,
            "name": self.name,
            "alternate_names": self.alternate_names.as_slice(),
            "origin_gen": self.origin_gen,
            "typ": self.typ,
            "gender": self.gender,
            "level": self.level,
            "abilities": self.abilities,
            "moves": self.moves,
        }
    }
}
