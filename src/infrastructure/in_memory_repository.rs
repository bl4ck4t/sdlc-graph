use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::repository::GraphRepository;
use crate::domain::User;

pub struct InMemoryGraphRepository {
    users: RwLock<HashMap<String, User>>,
}

impl InMemoryGraphRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl GraphRepository for InMemoryGraphRepository {
    async fn create_user(&self, user: User) {
        let mut users = self.users.write().await;
        users.insert(user.id.clone(), user);
    }

    async fn get_user(&self, id: &str) -> Option<User> {
        let users = self.users.read().await;
        users.get(id).cloned()
    }
}