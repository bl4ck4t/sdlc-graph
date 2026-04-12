use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct  Repository {
    pub id: String,
    pub name: String
}

impl Repository {
    pub fn new(id: String, name: String) -> Self{
        Self { id, name}
    }
}