use std::sync::Arc;

use crate::{
    api::error::AppError,
    domain::{User, commit::Commit, repository::GraphRepository, repository_entity::Repository},
};

pub struct GraphService {
    repo: Arc<dyn GraphRepository>,
}

impl GraphService {
    pub fn new(repo: Arc<dyn GraphRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, user: User) -> Result<User, AppError> {
        self.repo.create_user(user.clone()).await?;
        Ok(user)
    }

    pub async fn get_user(&self, id: &str) -> Result<User, AppError> {
        self.repo.get_user(id).await
    }

    pub async fn create_repository(&self, repo: Repository) -> Result<Repository, AppError> {
        self.repo.create_repository(repo.clone()).await?;
        Ok(repo)
    }

    pub async fn get_repository(&self, id: &str) -> Result<Repository, AppError> {
        self.repo.get_repository(id).await
    }

    pub async fn create_commit(&self, commit: Commit) -> Result<Commit, AppError> {
        self.repo.create_commit(commit.clone()).await?;
        Ok(commit)
    }

    pub async fn get_commit(&self, id: &str) -> Result<Commit, AppError> {
        self.repo.get_commit(id).await
    }

    pub async fn link_commit_to_user(
        &self,
        commit_id: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        self.repo.link_commit_to_user(commit_id, user_id).await
    }

    pub async fn link_commit_to_repository(
        &self,
        commit_id: &str,
        repo_id: &str,
    ) -> Result<(), AppError> {
        self.repo
            .link_commit_to_repository(commit_id, repo_id)
            .await
    }

    pub async fn get_commits_by_user(&self, user_id: &str) -> Result<Vec<Commit>, AppError> {
        self.repo.get_commits_by_user(user_id).await
    }

    pub async fn get_commits_by_repository(&self, repo_id: &str) -> Result<Vec<Commit>, AppError> {
        self.repo.get_commits_by_repository(repo_id).await
    }
}
