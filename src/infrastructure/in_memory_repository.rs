use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::commit::Commit;
use crate::domain::repository::{self, GraphRepository};
use crate::domain::User;
use crate::domain::repository_entity::Repository;

pub struct InMemoryGraphRepository {
    users: RwLock<HashMap<String, User>>,
    repositories: RwLock<HashMap<String, Repository>>,
    commits: RwLock<HashMap<String, Commit>>
}

impl InMemoryGraphRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            repositories: RwLock::new(HashMap::new()),
            commits: RwLock::new(HashMap::new())
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

    async fn create_repository(&self, repo: Repository) {
        let mut repos = self.repositories.write().await;
        repos.insert(repo.id.clone(), repo);
    }

    async fn get_repository(&self, id: &str) -> Option<Repository> {
        let repos = self.repositories.read().await;
        repos.get(id).cloned()
    }

    async fn create_commit(&self, commit: Commit) {
        let mut commits = self.commits.write().await;
        commits.insert(commit.id.clone(), commit);
    }

    async fn get_commit(&self, id: &str) -> Option<Commit> {
        let commits = self.commits.read().await;
        commits.get(id).cloned()
    }
}