use serde::{
    de::{self, Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use thiserror::Error;

use crate::utils::traits::SerializeEnum;

#[derive(Clone, Debug, sqlx::Type, PartialEq, PartialOrd)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Customer,
    Admin,
}

impl SerializeEnum<InvalidUserRole> for UserRole {
    type Error = InvalidUserRole;
    fn to_string(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Customer => "customer",
        }
    }

    fn from_str(s: &str) -> Result<Self, InvalidUserRole> {
        match s {
            "admin" => Ok(Self::Admin),
            "customer" => Ok(Self::Customer),
            _ => Err(InvalidUserRole::InvalidUser {
                role: String::from(s),
            }),
        }
    }
}

impl Serialize for UserRole {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_string())
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum InvalidUserRole {
    #[error("invalid user role provided: {role}")]
    InvalidUser { role: String },
}

struct UserRoleVisitor;

impl<'de> Deserialize<'de> for UserRole {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UserRoleVisitor)
    }
}

impl<'de> Visitor<'de> for UserRoleVisitor {
    type Value = UserRole;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "user role to a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        UserRole::from_str(v)
            .map_err(|_| serde::de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }

    fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        UserRole::from_str(v)
            .map_err(|_| serde::de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}
