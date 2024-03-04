use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "stat")]
pub enum Stat {
    HP,
    Attack,
    Defense,
    SpAttack,
    SpDefense,
    Speed,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum StatError {
    #[error("invalid stat provided: {unknown}")]
    InvalidStat { unknown: String },
}

impl SerDeserEnum for Stat {
    type Error = StatError;

    fn to_str(&self) -> &'static str {
        match self {
            Stat::HP => "hp",
            Stat::Attack => "attack",
            Stat::Defense => "defense",
            Stat::SpAttack => "sp_attack",
            Stat::SpDefense => "sp_defense",
            Stat::Speed => "speed",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "hp" => Ok(Stat::HP),
            "attack" => Ok(Stat::Attack),
            "defense" => Ok(Stat::Defense),
            "sp_attack" => Ok(Stat::SpAttack),
            "sp_defense" => Ok(Stat::SpDefense),
            "speed" => Ok(Stat::Speed),
            _ => Err(StatError::InvalidStat {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for Stat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct StatVisitor;

impl<'de> Deserialize<'de> for Stat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StatVisitor)
    }
}

impl<'de> Visitor<'de> for StatVisitor {
    type Value = Stat;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Stat' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Stat::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Stat::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StatWithVal {
    Attack(i32),
    Defense(i32),
    SpAttack(i32),
    SpDefense(i32),
    Speed(i32),
}
