use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::traits::IntoDocument;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            name,
            email,
            password,
        }
    }
}

impl IntoDocument for User {
    fn into_doc(self) -> mongodb::bson::Document {
        doc! {
            "name": self.name,
            "email": self.email,
            "password": self.password,
        }
    }
}
