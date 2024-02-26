use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use thiserror::Error;

#[derive(Clone, Debug)]
pub enum Gender {
    Male,
    Female
}

impl Gender {
    fn to_str(&self) -> &'static str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female"
        }
    }

    fn from_str(s: &str) -> Result<Self, GenderError> {
        match s.to_lowercase().as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            _ => Err( GenderError::InvalidGender { unknown: String::from(s) })
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GenderError {
    #[error("Invalid option provided for gender: {unknown}")]
    InvalidGender { unknown: String }
}

impl Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self.to_str())
    }
}

struct GenderVisitor;

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(GenderVisitor)
    }
}

impl<'de> Visitor<'de> for GenderVisitor {
    type Value = Gender;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "gender written as a string.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match Gender::from_str(v) {
            Ok(s) => Ok(s),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}