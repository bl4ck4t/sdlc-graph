use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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