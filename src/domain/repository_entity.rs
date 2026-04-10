use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Repository {
    pub id: String,
    pub name: String
}

impl Repository {
    pub fn new(id: String, name: String) -> Self{
        Self { id, name}
    }
}