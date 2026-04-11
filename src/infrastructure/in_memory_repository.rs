use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::api::error::AppError;
use crate::domain::commit::Commit;
use crate::domain::repository::{GraphRepository};
use crate::domain::User;
use crate::domain::repository_entity::Repository;

pub struct InMemoryGraphRepository {
    users: RwLock<HashMap<String, User>>,
    repositories: RwLock<HashMap<String, Repository>>,
    commits: RwLock<HashMap<String, Commit>>,

    //Relationships
    commit_to_repo: RwLock<HashMap<String, String>>,
    commit_to_user: RwLock<HashMap<String, String>>
}

impl InMemoryGraphRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            repositories: RwLock::new(HashMap::new()),
            commits: RwLock::new(HashMap::new()),
            commit_to_repo: RwLock::new(HashMap::new()),
            commit_to_user: RwLock::new(HashMap::new())
        }
    }
}

#[async_trait::async_trait]
impl GraphRepository for InMemoryGraphRepository {
    async fn create_user(&self, user: User) -> Result<(), AppError> {
        let mut users = self.users.write().await;
        
        if users.contains_key(&user.id) {
            return Err(AppError::UserAlreadyExists);
        }
        
        users.insert(user.id.clone(), user);

        Ok(())
    }

    async fn get_user(&self, id: &str) -> Result<User, AppError> {
        let users = self.users.read().await;
        users
            .get(id)
            .cloned()
            .ok_or(AppError::UserNotFound)
    }

    async fn create_repository(&self, repo: Repository) {
        let mut repos = self.repositories.write().await;
        repos.insert(repo.id.clone(), repo);
    }

    async fn get_repository(&self, id: &str) -> Result<Repository, AppError> {
        let repos = self.repositories.read().await;
        repos
            .get(id)
            .cloned()
            .ok_or(AppError::RepositoryNotFound)
    }

    async fn create_commit(&self, commit: Commit) {
        let mut commits = self.commits.write().await;
        commits.insert(commit.id.clone(), commit);
    }

    async fn get_commit(&self, id: &str) -> Result<Commit, AppError> {
        let commits = self.commits.read().await;
        commits
        .get(id)
        .cloned()
        .ok_or(AppError::CommitNotFound)
    }

    async fn link_commit_to_repository(&self, commit_id: &str, repo_id: &str) -> Result<(), AppError> {
        let commits = self.commits.read().await;
        if !commits.contains_key(commit_id) {
            return Err(AppError::CommitNotFound);
        }
        drop(commits);

        let repos = self.repositories.read().await;
        if !repos.contains_key(repo_id) {
            return Err(AppError::RepositoryNotFound);
        }
        drop(repos);

        let mut map = self.commit_to_repo.write().await;
        map.insert(commit_id.to_string(), repo_id.to_string());

        Ok(())
    }

    async fn link_commit_to_user(&self, commit_id: &str, user_id: &str) -> Result<(), AppError> {
        let commits = self.commits.read().await;
        if !commits.contains_key(commit_id) {
            return Err(AppError::CommitNotFound);
        }
        drop(commits);

        let users = self.users.read().await;
        if !users.contains_key(user_id) {
            return Err(AppError::UserNotFound);
        }
        drop(users);

        let mut map = self.commit_to_user.write().await;
        map.insert(commit_id.to_string(), user_id.to_string());

        Ok(())
    }

    async fn get_commits_by_repository(&self, repo_id: &str) -> Result<Vec<Commit>, AppError> {

        let repos = self.repositories.read().await;
        if !repos.contains_key(repo_id) {
            return Err(AppError::RepositoryNotFound);
        }
        drop(repos);


        let map = self.commit_to_repo.read().await;
        let commits = self.commits.read().await;

        let result = map.iter()
            .filter(|(_, r_id)| r_id == &repo_id)
            .filter_map(|(c_id, _)| commits.get(c_id).cloned())
            .collect();

        Ok(result)
    }

    async fn get_commits_by_user(&self, user_id: &str) -> Result<Vec<Commit>, AppError> {

        let users = self.users.read().await;
        if !users.contains_key(user_id) {
            return Err(AppError::UserNotFound);
        }
        drop(users);
        
        let map = self.commit_to_user.read().await;
        let commits = self.commits.read().await;

        let result = map
            .iter()
            .filter(|(_, u_id)| u_id == &user_id)
            .filter_map(|(c_id, _)| commits.get(c_id).cloned())
            .collect();

        Ok(result)
    }
}