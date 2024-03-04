use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "region")]
pub enum Region {
    Kanto,
    Johto,
    Hoenn,
    Hisui,
    Sinnoh,
    Unova,
    Kalos,
    Alola,
    Galar,
    Paldea,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RegionError {
    #[error("invalid region provided: {unknown}")]
    InvalidRegion { unknown: String },
}

impl SerDeserEnum for Region {
    type Error = RegionError;

    fn to_str(&self) -> &'static str {
        match self {
            Region::Kanto => "kanto",
            Region::Johto => "johto",
            Region::Hoenn => "hoenn",
            Region::Hisui => "hisui",
            Region::Sinnoh => "sinnoh",
            Region::Unova => "unova",
            Region::Kalos => "kalos",
            Region::Alola => "alola",
            Region::Galar => "galar",
            Region::Paldea => "paldea",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "kanto" => Ok(Region::Kanto),
            "johto" => Ok(Region::Johto),
            "hoenn" => Ok(Region::Hoenn),
            "hisui" => Ok(Region::Hisui),
            "sinnoh" => Ok(Region::Sinnoh),
            "unova" => Ok(Region::Unova),
            "kalos" => Ok(Region::Kalos),
            "alola" => Ok(Region::Alola),
            "galar" => Ok(Region::Galar),
            "paldea" => Ok(Region::Paldea),
            _ => Err(RegionError::InvalidRegion {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct RegionVisitor;

impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RegionVisitor)
    }
}

impl<'de> Visitor<'de> for RegionVisitor {
    type Value = Region;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Region' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Region::from_str(v) {
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
        match Region::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}
