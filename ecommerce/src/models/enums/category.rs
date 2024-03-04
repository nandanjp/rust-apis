use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use thiserror::Error;

use crate::utils::traits::SerializeEnum;

#[derive(Clone, Debug, sqlx::Type, PartialEq, PartialOrd)]
#[sqlx(type_name = "category", rename_all = "lowercase")]
pub enum Category {
    Smartphones,
    Laptops,
    Tablets,
    Cameras,
    Iems,
    Headphones,
    VideoGames,
    Consoles,
    MusicAlbums,
    Movies,
    Figures,
    PlayingCards,
}

impl SerializeEnum<CategoryError> for Category {
    type Error = CategoryError;
    fn to_string(&self) -> &'static str {
        match self {
            Category::Smartphones => "smartphones",
            Category::Laptops => "laptops",
            Category::Tablets => "tablets",
            Category::Cameras => "cameras",
            Category::Iems => "iems",
            Category::Headphones => "headphones",
            Category::VideoGames => "videogames",
            Category::Consoles => "consoles",
            Category::MusicAlbums => "musicalbums",
            Category::Movies => "movies",
            Category::Figures => "figures",
            Category::PlayingCards => "playingcards",
        }
    }

    fn from_str(s: &str) -> Result<Self, CategoryError> {
        match s.to_lowercase().as_str() {
            "smartphones" => Ok(Category::Smartphones),
            "laptops" => Ok(Category::Laptops),
            "tablets" => Ok(Category::Tablets),
            "cameras" => Ok(Category::Cameras),
            "iems" => Ok(Category::Iems),
            "headphones" => Ok(Category::Headphones),
            "video_games" => Ok(Category::VideoGames),
            "consoles" => Ok(Category::Consoles),
            "musicalbums" => Ok(Category::MusicAlbums),
            "movies" => Ok(Category::Movies),
            "figures" => Ok(Category::Figures),
            "playingcards" => Ok(Category::PlayingCards),
            _ => Err(CategoryError::InvalidCategory {
                unknown: s.to_string(),
            }),
        }
    }
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum CategoryError {
    #[error("invalid category name provided: {unknown}")]
    InvalidCategory { unknown: String },
}

struct CategoryVisitor;

impl Serialize for Category {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self.to_string())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(CategoryVisitor)
    }
}

impl<'de> Visitor<'de> for CategoryVisitor {
    type Value = Category;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "category to a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Category::from_str(v)
            .map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self))
    }

    fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Category::from_str(v)
            .map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self))
    }
}
