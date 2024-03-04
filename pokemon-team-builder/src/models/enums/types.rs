use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "type")]
pub enum Type {
    Normal,
    Fire,
    Water,
    Grass,
    Flying,
    Fighting,
    Poison,
    Electric,
    Ground,
    Rock,
    Psychic,
    Ice,
    Bug,
    Ghost,
    Steel,
    Dragon,
    Dark,
    Fairy,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum TypeError {
    #[error("invalid type provided: {unknown}")]
    InvalidType { unknown: String },
}

impl Type {
    pub fn no_damage_to(&self) -> TypeRelations {
        TypeRelations::NoDamageTo(Vec::new())
    }

    pub fn half_damage_to(&self) -> TypeRelations {
        TypeRelations::HalfDamageTo(Vec::new())
    }

    pub fn double_damage_to(&self) -> TypeRelations {
        TypeRelations::DoubleDamageTo(Vec::new())
    }

    pub fn no_damage_from(&self) -> TypeRelations {
        TypeRelations::NoDamageFrom(Vec::new())
    }

    pub fn half_damage_from(&self) -> TypeRelations {
        TypeRelations::HalfDamageFrom(Vec::new())
    }

    pub fn double_damage_from(&self) -> TypeRelations {
        TypeRelations::DoubleDamageFrom(Vec::new())
    }
}

impl SerDeserEnum for Type {
    type Error = TypeError;

    fn to_str(&self) -> &'static str {
        match self {
            Type::Bug => "bug",
            Type::Dark => "dark",
            Type::Dragon => "dragon",
            Type::Electric => "electric",
            Type::Fairy => "fairy",
            Type::Fighting => "fighting",
            Type::Fire => "fire",
            Type::Flying => "flying",
            Type::Ghost => "ghost",
            Type::Grass => "grass",
            Type::Ground => "ground",
            Type::Ice => "ice",
            Type::Normal => "normal",
            Type::Poison => "poison",
            Type::Psychic => "psychic",
            Type::Rock => "rock",
            Type::Steel => "steel",
            Type::Water => "water",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "bug" => Ok(Type::Bug),
            "dark" => Ok(Type::Dark),
            "dragon" => Ok(Type::Dragon),
            "electric" => Ok(Type::Electric),
            "fairy" => Ok(Type::Fairy),
            "fighting" => Ok(Type::Fighting),
            "fire" => Ok(Type::Fire),
            "flying" => Ok(Type::Flying),
            "ghost" => Ok(Type::Ghost),
            "grass" => Ok(Type::Grass),
            "ground" => Ok(Type::Ground),
            "ice" => Ok(Type::Ice),
            "normal" => Ok(Type::Normal),
            "poison" => Ok(Type::Poison),
            "psychic" => Ok(Type::Psychic),
            "rock" => Ok(Type::Rock),
            "steel" => Ok(Type::Steel),
            "water" => Ok(Type::Water),
            _ => Err(TypeError::InvalidType {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct TypeVisitor;

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TypeVisitor)
    }
}

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Type' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Type::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Type::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}

#[derive(serde::Serialize)]
pub enum TypeRelations {
    NoDamageTo(Vec<Type>),
    HalfDamageTo(Vec<Type>),
    DoubleDamageTo(Vec<Type>),
    NoDamageFrom(Vec<Type>),
    HalfDamageFrom(Vec<Type>),
    DoubleDamageFrom(Vec<Type>),
}
