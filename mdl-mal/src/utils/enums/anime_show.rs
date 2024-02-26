use std::fmt;
use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use thiserror::Error;

#[derive(Clone, Debug)]
pub enum AnimeOrShow {
    Anime,
    Show
}
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AnimeOrShowError {
    #[error("Invalid option for a show or anime: {unknown}")]
    InvalidAnimeOrShow { unknown: String }
}

impl AnimeOrShow {
    fn to_str(&self) -> &'static str {
        match self {
            AnimeOrShow::Anime => "anime",
            AnimeOrShow::Show => "show"
        }
    }

    fn from_str(s: & str) -> Result<Self, AnimeOrShowError> {
        match s.to_lowercase().as_str() {
            "anime" => Ok(AnimeOrShow::Anime),
            "show" => Ok(AnimeOrShow::Show),
            _ => Err(AnimeOrShowError::InvalidAnimeOrShow { unknown: String::from(s) })
        }
    }
}

struct AnimeOrShowVisitor;
impl Serialize for AnimeOrShow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for AnimeOrShow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(AnimeOrShowVisitor)
    }
}

impl<'de> Visitor<'de> for AnimeOrShowVisitor {
    type Value = AnimeOrShow;
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "anime or show written as a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
        match AnimeOrShow::from_str(v) {
            Ok(s) => Ok(s),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: de::Error,
    {
        match AnimeOrShow::from_str(v) {
            Ok(x) => Ok(x),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
}