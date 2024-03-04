use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::traits::{IntoSerializable, SerializeEnum};

use super::enums::user_role::UserRole;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub users_role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoSerializable<UserSerializable> for User {
    fn to_serial(self) -> UserSerializable {
        UserSerializable {
            user_id: self.id,
            username: self.username,
            password: self.password,
            email: self.email,
            address: self.address,
            users_role: String::from(self.users_role.to_string()),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct UserSerializable {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub users_role: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub role: UserRole,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct UserResponse {
    pub success: bool,
    pub user: Option<UserSerializable>,
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ListUserResponse {
    pub success: bool,
    pub users: Option<Vec<UserSerializable>>,
    pub error: Option<String>,
}
