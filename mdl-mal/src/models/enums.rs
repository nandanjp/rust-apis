use std::fmt;
use isocountry::CountryCode;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Gender {
    Male,
    Female
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserType {
    Admin,
    User
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OriginCountry {
    Japan,
    SouthKorea,
    China,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Rating {
    Zero,
    Half,
    One,
    OneHalf,
    Two,
    TwoHalf,
    Three,
    ThreeHalf,
    Four,
    FourHalf,
    Five,
    FiveHalf,
    Six,
    SixHalf,
    Seven,
    SevenHalf,
    Eight,
    EightHalf,
    Nine,
    NineHalf,
    Ten
}

#[derive(Debug, Error, PartialEq)]
pub enum RatingError {
    #[error("Invalid rating provided: {unknown}")]
    InvalidError { unknown: String }
}

impl Rating {
    pub fn str_from_rating(&self) -> &'static str {
        match self {
            Rating::Zero => "0",
            Rating::Half => "0.5",
            Rating::One => "1",
            Rating::OneHalf => "1.5",
            Rating::Two => "2",
            Rating::TwoHalf => "2.5",
            Rating::Three => "3",
            Rating::ThreeHalf => "3.5",
            Rating::Four => "4",
            Rating::FourHalf => "4.5",
            Rating::Five => "5",
            Rating::FiveHalf => "5.5",
            Rating::Six => "6",
            Rating::SixHalf => "6.5",
            Rating::Seven => "7",
            Rating::SevenHalf => "7.5",
            Rating::Eight => "8",
            Rating::EightHalf => "8.5",
            Rating::Nine => "9",
            Rating::NineHalf => "9.5",
            Rating::Ten => "10"
        }
    }

    pub fn rating_from_str(s: &str) -> Result<Self, RatingError> {
        match s {
            "0" => Ok(Self::Zero),
            "0.5" => Ok(Self::Half),
            "1" => Ok(Self::One),
            "1.5" => Ok(Self::OneHalf),
            "2" => Ok(Self::Two),
            "2.5" => Ok(Self::TwoHalf),
            "3" => Ok(Self::Three),
            "3.5" => Ok(Self::ThreeHalf),
            "4" => Ok(Self::Four),
            "4.5" => Ok(Self::FourHalf),
            "5" => Ok(Self::Five),
            "5.5" => Ok(Self::FiveHalf),
            "6" => Ok(Self::Six),
            "6.5" => Ok(Self::SixHalf),
            "7" => Ok(Self::Seven),
            "7.5" => Ok(Self::SevenHalf),
            "8" => Ok(Self::Eight),
            "8.5" => Ok(Self::EightHalf),
            "9" => Ok(Self::Nine),
            "9.5" => Ok(Self::NineHalf),
            "10" => Ok(Self::Ten),
            _ => Err(RatingError::InvalidError { unknown: format!("{s} is not an accepted rating") })
        }
    }
}

struct RatingVisitor;

impl Serialize for Rating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.collect_str(self.str_from_rating())
    }
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(RatingVisitor)
    }
}

impl<'de> Visitor<'de> for RatingVisitor {
    type Value = Rating;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a rating written as a string value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match Rating::rating_from_str(v) {
            Ok(x) => Ok(x),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match Rating::rating_from_str(v) {
            Ok(x) => Ok(x),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
}