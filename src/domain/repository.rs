use crate::{api::error::AppError, domain::{User, commit::Commit, repository_entity::Repository}};

#[async_trait::async_trait]
pub trait GraphRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<(), AppError>;
    async fn get_user(&self, id: &str) -> Result<User, AppError>;

    async fn create_repository(&self, repo: Repository);
    async fn get_repository(&self, id: &str) -> Result<Repository, AppError>;

    async fn create_commit(&self, commit: Commit);
    async fn get_commit(&self, id: &str) -> Result<Commit, AppError>;

    async fn link_commit_to_repository(&self, commit_id: &str, repo_id: &str) -> Result<(), AppError>;
    async fn link_commit_to_user(&self, commit_id: &str, user_id: &str) -> Result<(), AppError>;
    async fn get_commits_by_repository(&self, repo_id: &str) -> Result<Vec<Commit>, AppError>;
    async fn get_commits_by_user(&self, user_id: &str) -> Result<Vec<Commit>, AppError>;
}