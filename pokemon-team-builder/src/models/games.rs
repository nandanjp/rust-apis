use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::IntoSerial;

use super::enums::{region::Region, types::Type};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Generation {
    pub id: i32,
    pub name: String,
    pub main_region: Region,
    pub types: Vec<Type>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for Generation {
    type Serial = GenerationSerial;

    fn to_serial(&self) -> Self::Serial {
        GenerationSerial {
            id: self.id,
            name: self.name.clone(),
            main_region: self.main_region.clone(),
            types: self.types.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GenerationSerial {
    pub id: i32,
    pub name: String,
    pub main_region: Region,
    pub types: Vec<Type>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateGeneration {
    pub name: String,
    pub main_region: Region,
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, FromRow)]
pub struct Pokedex {
    pub id: i32,
    pub name: String,
    pub is_main_series: bool,
    pub description: String,
    pub region: Region,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerial for Pokedex {
    type Serial = PokedexSerial;

    fn to_serial(&self) -> Self::Serial {
        PokedexSerial {
            id: self.id,
            name: self.name.clone(),
            is_main_series: self.is_main_series,
            description: self.description.clone(),
            region: self.region.clone(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PokedexSerial {
    pub id: i32,
    pub name: String,
    pub is_main_series: bool,
    pub description: String,
    pub region: Region,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreatePokedex {
    pub name: String,
    pub is_main_series: bool,
    pub description: String,
    pub region: Region,
}
