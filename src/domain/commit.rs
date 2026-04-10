use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String
}

impl Commit {
    pub fn new(id: String, message: String) -> Self {
        Self { id, message }
    }
}