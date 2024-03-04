use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GenderError {
    #[error("invalid gender provided: {unknown}")]
    InvalidGender { unknown: String },
}

impl SerDeserEnum for Gender {
    type Error = GenderError;

    fn to_str(&self) -> &'static str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Unknown => "unknown",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "unknown" => Ok(Gender::Unknown),
            _ => Err(GenderError::InvalidGender {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct GenderVisitor;

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(GenderVisitor)
    }
}

impl<'de> Visitor<'de> for GenderVisitor {
    type Value = Gender;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Gender' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Gender::from_str(v) {
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
        match Gender::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}
