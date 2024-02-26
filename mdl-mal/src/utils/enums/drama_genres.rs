use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum DramaGenres {
    Action,
    Animals,
    Comedy,
    Detective,
    Drama,
    Fantasy,
    Friendship,
    Horror,
    Law,
    Manga,
    Mature,
    Melodrama,
    Music,
    Political,
    Romance,
    SciFi,
    Sports,
    Suspense,
    Tokusatsu,
    Vampire,
    Western,
    Youth,
    Adventure,
    Business,
    Crime,
    Documentary,
    Family,
    Food,
    Historical,
    Investigation,
    Life,
    MartialArts,
    Medical,
    Military,
    Mystery,
    Psychological,
    School,
    Sitcom,
    Supernatural,
    Thriller,
    Tragedy,
    War,
    Wuxia,
    Zombies
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DramaGenresError {
    #[error("Invalid drama genre provided: {unknown}")]
    InvalidDramaGenre { unknown: String }
}

impl DramaGenres {
    fn to_str(&self) -> &'static str {
        match self {
            DramaGenres::Action => "action",
            DramaGenres::Animals => "animals",
            DramaGenres::Comedy => "comedy",
            DramaGenres::Detective => "detective",
            DramaGenres::Drama => "drama",
            DramaGenres::Fantasy => "fantasy",
            DramaGenres::Friendship => "friendship",
            DramaGenres::Horror => "horror",
            DramaGenres::Law => "law",
            DramaGenres::Manga => "manga",
            DramaGenres::Mature => "mature",
            DramaGenres::Melodrama => "melodrama",
            DramaGenres::Music => "music",
            DramaGenres::Political => "political",
            DramaGenres::Romance => "romance",
            DramaGenres::SciFi => "sci_fi",
            DramaGenres::Sports => "sports",
            DramaGenres::Suspense => "suspense",
            DramaGenres::Tokusatsu => "tokusatsu",
            DramaGenres::Vampire => "vampire",
            DramaGenres::Western => "western",
            DramaGenres::Youth => "youth",
            DramaGenres::Adventure => "adventure",
            DramaGenres::Business => "business",
            DramaGenres::Crime => "crime",
            DramaGenres::Documentary => "documentary",
            DramaGenres::Family => "family",
            DramaGenres::Food => "food",
            DramaGenres::Historical => "historical",
            DramaGenres::Investigation => "investigation",
            DramaGenres::Life => "life",
            DramaGenres::MartialArts => "martial_arts",
            DramaGenres::Medical => "medical",
            DramaGenres::Military => "military",
            DramaGenres::Mystery => "mystery",
            DramaGenres::Psychological => "psychological",
            DramaGenres::School => "school",
            DramaGenres::Sitcom => "sitcom",
            DramaGenres::Supernatural => "supernatural",
            DramaGenres::Thriller => "thriller",
            DramaGenres::Tragedy => "tragedy",
            DramaGenres::War => "war",
            DramaGenres::Wuxia => "wuxia",
            DramaGenres::Zombies => "zombies"
        }
    }
    fn from_str(s: &str) -> Result<Self, DramaGenresError> {
        match s {
            "action" => Ok(DramaGenres::Action),
            "animals" => Ok(DramaGenres::Animals),
            "comedy" => Ok(DramaGenres::Comedy),
            "detective" => Ok(DramaGenres::Detective),
            "drama" => Ok(DramaGenres::Drama),
            "fantasy" => Ok(DramaGenres::Fantasy),
            "friendship" => Ok(DramaGenres::Friendship),
            "horror" => Ok(DramaGenres::Horror),
            "law" => Ok(DramaGenres::Law),
            "manga" => Ok(DramaGenres::Manga),
            "mature" => Ok(DramaGenres::Mature),
            "melodrama" => Ok(DramaGenres::Melodrama),
            "music" => Ok(DramaGenres::Music),
            "political" => Ok(DramaGenres::Political),
            "romance" => Ok(DramaGenres::Romance),
            "sci_fi" => Ok(DramaGenres::SciFi),
            "sports" => Ok(DramaGenres::Sports),
            "suspense" => Ok(DramaGenres::Suspense),
            "tokusatsu" => Ok(DramaGenres::Tokusatsu),
            "vampire" => Ok(DramaGenres::Vampire),
            "western" => Ok(DramaGenres::Western),
            "youth" => Ok(DramaGenres::Youth),
            "adventure" => Ok(DramaGenres::Adventure),
            "business" => Ok(DramaGenres::Business),
            "crime" => Ok(DramaGenres::Crime),
            "documentary" => Ok(DramaGenres::Documentary),
            "family" => Ok(DramaGenres::Family),
            "food" => Ok(DramaGenres::Food),
            "historical" => Ok(DramaGenres::Historical),
            "investigation" => Ok(DramaGenres::Investigation),
            "life" => Ok(DramaGenres::Life),
            "martial_arts" => Ok(DramaGenres::MartialArts),
            "medical" => Ok(DramaGenres::Medical),
            "military" => Ok(DramaGenres::Military),
            "mystery" => Ok(DramaGenres::Mystery),
            "psychological" => Ok(DramaGenres::Psychological),
            "school" => Ok(DramaGenres::School),
            "sitcom" => Ok(DramaGenres::Sitcom),
            "supernatural" => Ok(DramaGenres::Supernatural),
            "thriller" => Ok(DramaGenres::Thriller),
            "tragedy" => Ok(DramaGenres::Tragedy),
            "war" => Ok(DramaGenres::War),
            "wuxia" => Ok(DramaGenres::Wuxia),
            "zombies" => Ok(DramaGenres::Zombies),
            _ => Err(DramaGenresError::InvalidDramaGenre { unknown: String::from(s) })
        }
    }
}

struct DramaGenresVisitor;

impl Serialize for DramaGenres {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.collect_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for DramaGenres {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(DramaGenresVisitor)
    }
}

impl<'de> Visitor<'de> for DramaGenresVisitor {
    type Value = DramaGenres;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "drama genre as a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match DramaGenres::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        match DramaGenres::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}