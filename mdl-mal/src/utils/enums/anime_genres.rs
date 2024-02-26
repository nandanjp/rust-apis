use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum AnimeGenres {
    Action,
    Adventure,
    AvantGarde,
    AwardWinning,
    Comedy,
    Drama,
    Fantasy,
    Gourmet,
    Horror,
    Mystery,
    Romance,
    SciFi,
    SliceOfLife,
    Sports,
    Supernatural,
    Suspense
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AnimeGenresError {
    #[error("Invalid anime genre provided: {unknown}")]
    InvalidAnimeGenre {unknown: String}
}

impl AnimeGenres {
    pub fn to_str(&self) -> &'static str {
        match self {
            AnimeGenres::Action => "action",
            AnimeGenres::Adventure => "adventure",
            AnimeGenres::AvantGarde => "avant_garde",
            AnimeGenres::AwardWinning => "award_winning",
            AnimeGenres::Comedy => "comedy",
            AnimeGenres::Drama => "drama",
            AnimeGenres::Fantasy => "fantasy",
            AnimeGenres::Gourmet => "gourmet",
            AnimeGenres::Horror => "horror",
            AnimeGenres::Mystery => "mystery",
            AnimeGenres::Romance => "romance",
            AnimeGenres::SciFi => "sci_fi",
            AnimeGenres::SliceOfLife => "slice_of_life",
            AnimeGenres::Sports => "sports",
            AnimeGenres::Supernatural => "supernatural",
            AnimeGenres::Suspense => "suspense",
        }
    }
    pub fn from_str(s: &str) -> Result<Self, AnimeGenresError> {
        match s.to_lowercase().as_str() {
            "action" => Ok(AnimeGenres::Action),
            "adventure" => Ok(AnimeGenres::Adventure),
            "avant_garde" => Ok(AnimeGenres::AvantGarde),
            "award_winning" => Ok(AnimeGenres::AwardWinning),
            "comedy" => Ok(AnimeGenres::Comedy),
            "drama" => Ok(AnimeGenres::Drama),
            "fantasy" => Ok(AnimeGenres::Fantasy),
            "gourmet" => Ok(AnimeGenres::Gourmet),
            "horror" => Ok(AnimeGenres::Horror),
            "mystery" => Ok(AnimeGenres::Mystery),
            "romance" => Ok(AnimeGenres::Romance),
            "sci_fi" => Ok(AnimeGenres::SciFi),
            "slice_of_life" => Ok(AnimeGenres::SliceOfLife),
            "sports" => Ok(AnimeGenres::Sports),
            "supernatural" => Ok(AnimeGenres::Supernatural),
            "suspense" => Ok(AnimeGenres::Suspense),
            _ => Err(AnimeGenresError::InvalidAnimeGenre {unknown: String::from(s)})
        }
    }
}

impl Serialize for AnimeGenres {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self.to_str())
    }
}

struct AnimeGenresVisitor;

impl<'de> Deserialize<'de> for AnimeGenres {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(AnimeGenresVisitor)
    }
}

impl<'de> Visitor<'de> for AnimeGenresVisitor {
    type Value = AnimeGenres;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "an anime genre written as a string.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match AnimeGenres::from_str(v) {
            Ok(s) => Ok(s),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        match AnimeGenres::from_str(v) {
            Ok(s) => Ok(s),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}