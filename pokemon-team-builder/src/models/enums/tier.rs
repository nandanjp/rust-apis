use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "tier")]
pub enum Tier {
    AG,
    Ubers,
    OU,
    UU,
    RU,
    NU,
    PU,
    ZU,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum TierError {
    #[error("invalid tier provided: {unknown}")]
    InvalidTier { unknown: String },
}

impl SerDeserEnum for Tier {
    type Error = TierError;

    fn to_str(&self) -> &'static str {
        match self {
            Tier::AG => "ag",
            Tier::Ubers => "ubers",
            Tier::OU => "ou",
            Tier::UU => "uu",
            Tier::RU => "ru",
            Tier::NU => "nu",
            Tier::PU => "pu",
            Tier::ZU => "zu",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "ag" => Ok(Tier::AG),
            "ubers" => Ok(Tier::Ubers),
            "ou" => Ok(Tier::OU),
            "uu" => Ok(Tier::UU),
            "ru" => Ok(Tier::RU),
            "nu" => Ok(Tier::NU),
            "pu" => Ok(Tier::PU),
            "zu" => Ok(Tier::ZU),
            _ => Err(TierError::InvalidTier {
                unknown: String::from(s),
            }),
        }
    }
}

struct TierVisitor;

impl Serialize for Tier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Tier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TierVisitor)
    }
}

impl<'de> Visitor<'de> for TierVisitor {
    type Value = Tier;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Tier' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Tier::from_str(v) {
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
        match Tier::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}
