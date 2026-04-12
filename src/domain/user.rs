use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String
}

impl User {
    pub fn new(id: String, username: String, email: String) -> Self {
        Self { id, username, email }
    }
}