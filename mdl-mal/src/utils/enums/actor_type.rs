use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use thiserror::Error;

#[derive(Clone, Debug)]
pub enum ActorType {
    Actor,
    Director,
    ScreenWriter,
    Producer,
    MusicEditor,
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum ActorTypeError {
    #[error("Invalid actor type provided: {unknown}")]
    InvalidActorType {unknown: String}
}

impl ActorType {
    fn to_str(&self) -> &'static str {
        match self {
            ActorType::Actor => "actor",
            ActorType::Director => "director",
            ActorType::Producer => "producer",
            ActorType::ScreenWriter => "screen_writer",
            ActorType::MusicEditor => "music_editor"
        }
    }
    
    fn from_str(s: &str) -> Result<Self, ActorTypeError> {
        match s.to_lowercase().as_str() {
            "actor" => Ok(ActorType::Actor),
            "director" => Ok(ActorType::Director),
            "producer" => Ok(ActorType::Producer),
            "screen_writer" => Ok(ActorType::ScreenWriter),
            "music_editor" => Ok(ActorType::MusicEditor),
            _ => Err(ActorTypeError::InvalidActorType { unknown: String::from(s) })
        }
    }
}

pub struct ActorTypeVisitor;

impl Serialize for ActorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for ActorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(ActorTypeVisitor)
    }
}

impl<'de> Visitor<'de> for ActorTypeVisitor {
    type Value = ActorType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "actor type as a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match ActorType::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}