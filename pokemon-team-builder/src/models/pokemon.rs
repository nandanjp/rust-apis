use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use super::{
    abilities::Abilities,
    enums::{Gender, OriginGame, Stat, Type},
    traits::IntoDocument,
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
}

impl IntoDocument for Pokemon {
    fn into_doc(self) -> Document {
        doc! {
            "pokedex_id": self.pokedex_id as u32,
            "name": self.name,
            "alternate_names": self.alternate_names.as_slice(),
            "origin_gen": self.origin_gen.to_string(),
            "typ": self.typ.to_string(),
            "gender": self.gender.to_string(),
            "level": self.level.map_or(100_u32, |l| l as u32),
            "abilities": self.abilities.to_string(),
            "moves": self.moves,
        }
    }
}
