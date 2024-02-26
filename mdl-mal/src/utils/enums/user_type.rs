use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use thiserror::Error;

#[derive(Clone, Debug)]
pub enum UserType {
    Admin,
    User
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum UserTypeError {
    #[error("Invalid user type provided: {unknown}")]
    InvalidUserType { unknown: String }
}

impl UserType {
    fn to_str(&self) -> &'static str {
        match self {
            UserType::Admin => "admin",
            UserType::User => "user"
        }
    }

    fn from_str(s: &str) -> Result<Self, UserTypeError> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserType::Admin),
            "user" => Ok(UserType::User),
            _ => Err(UserTypeError::InvalidUserType { unknown: String::from(s) })
        }
    }
}

struct UserTypeVisitor;

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for UserType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(UserTypeVisitor)
    }
}

impl<'de> Visitor<'de> for UserTypeVisitor {
    type Value = UserType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "user type as a string.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match UserType::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}