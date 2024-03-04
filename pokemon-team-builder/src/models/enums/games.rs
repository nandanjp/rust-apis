use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

use crate::utils::traits::SerDeserEnum;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "game")]
pub enum Game {
    RedBlue,
    GoldSilver,
    RubySapphire,
    DiamondPearl,
    BlackWhite,
    XY,
    SunMoon,
    SwordShield,
    ScarletViolet,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum GameError {
    #[error("invalid game provided: {unknown}")]
    InvalidGame { unknown: String },
}

impl SerDeserEnum for Game {
    type Error = GameError;
    fn to_str(&self) -> &'static str {
        match self {
            Self::RedBlue => "redblue",
            Self::GoldSilver => "goldsilver",
            Self::RubySapphire => "rubysapphire",
            Self::DiamondPearl => "diamondpearl",
            Self::BlackWhite => "blackwhite",
            Self::XY => "xy",
            Self::SunMoon => "sunmoon",
            Self::SwordShield => "swordshield",
            Self::ScarletViolet => "scarletviolet",
        }
    }

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match s.to_lowercase().as_str() {
            "redblue" => Ok(Game::RedBlue),
            "goldsilver" => Ok(Game::GoldSilver),
            "rubysapphire" => Ok(Game::RubySapphire),
            "diamondpearl" => Ok(Game::DiamondPearl),
            "blackwhite" => Ok(Game::BlackWhite),
            "xy" => Ok(Game::XY),
            "sunmoon" => Ok(Game::SunMoon),
            "swordshield" => Ok(Game::SwordShield),
            "scarletviolet" => Ok(Game::ScarletViolet),
            _ => Err(GameError::InvalidGame {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct GameVisitor;

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(GameVisitor)
    }
}

impl<'de> Visitor<'de> for GameVisitor {
    type Value = Game;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'Game' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Game::from_str(v) {
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
        match Game::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}
