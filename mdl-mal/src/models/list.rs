use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::enums::Rating;
use crate::models::show::Show;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub user_id: ObjectId,
    pub shows: Vec<Show>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}