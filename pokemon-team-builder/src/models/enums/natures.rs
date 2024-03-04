use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

use super::stats::Stat;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "nature")]
pub enum Nature {
    Adamant,
    Bashful,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Gentle,
    Hardy,
    Hasty,
    Impish,
    Jolly,
    Lax,
    Lonely,
    Mild,
    Modest,
    Naive,
    Naughty,
    Quiet,
    Quirky,
    Rash,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum NatureError {
    #[error("invalid nature provided: {unknown}")]
    InvalidNature { unknown: String },
}

impl SerDeserEnum for Nature {
    type Error = NatureError;
    fn to_str(&self) -> &'static str {
        match self {
            Nature::Adamant => "adamant",
            Nature::Bashful => "bashful",
            Nature::Bold => "bold",
            Nature::Brave => "brave",
            Nature::Calm => "calm",
            Nature::Careful => "careful",
            Nature::Docile => "docile",
            Nature::Gentle => "gentle",
            Nature::Hardy => "hardy",
            Nature::Hasty => "hasty",
            Nature::Impish => "impish",
            Nature::Jolly => "jolly",
            Nature::Lax => "lax",
            Nature::Lonely => "lonely",
            Nature::Mild => "mild",
            Nature::Modest => "modest",
            Nature::Naive => "naive",
            Nature::Naughty => "naughty",
            Nature::Quiet => "quiet",
            Nature::Quirky => "quirky",
            Nature::Rash => "rash",
            Nature::Relaxed => "relaxed",
            Nature::Sassy => "sassy",
            Nature::Serious => "serious",
            Nature::Timid => "timid",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "adamant" => Ok(Nature::Adamant),
            "bashful" => Ok(Nature::Bashful),
            "bold" => Ok(Nature::Bold),
            "brave" => Ok(Nature::Brave),
            "calm" => Ok(Nature::Calm),
            "careful" => Ok(Nature::Careful),
            "docile" => Ok(Nature::Docile),
            "gentle" => Ok(Nature::Gentle),
            "hardy" => Ok(Nature::Hardy),
            "hasty" => Ok(Nature::Hasty),
            "impish" => Ok(Nature::Impish),
            "jolly" => Ok(Nature::Jolly),
            "lax" => Ok(Nature::Lax),
            "lonely" => Ok(Nature::Lonely),
            "mild" => Ok(Nature::Mild),
            "modest" => Ok(Nature::Modest),
            "naive" => Ok(Nature::Naive),
            "naughty" => Ok(Nature::Naughty),
            "quiet" => Ok(Nature::Quiet),
            "quirky" => Ok(Nature::Quirky),
            "rash" => Ok(Nature::Rash),
            "relaxed" => Ok(Nature::Relaxed),
            "sassy" => Ok(Nature::Sassy),
            "serious" => Ok(Nature::Serious),
            "timid" => Ok(Nature::Timid),
            _ => Err(NatureError::InvalidNature {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for Nature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct NatureVisitor;
impl<'de> Deserialize<'de> for Nature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NatureVisitor)
    }
}

impl<'de> Visitor<'de> for NatureVisitor {
    type Value = Nature;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Nature' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Nature::from_str(v) {
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
        match Nature::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct NatureWithStats(Nature, Stat, Stat);
