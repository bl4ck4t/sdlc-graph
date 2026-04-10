use crate::domain::{User, commit::Commit, repository_entity::Repository};

#[async_trait::async_trait]
pub trait GraphRepository: Send + Sync {
    async fn create_user(&self, user: User);
    async fn get_user(&self, id: &str) -> Option<User>;

    async fn create_repository(&self, repo: Repository);
    async fn get_repository(&self, id: &str) -> Option<Repository>;

    async fn create_commit(&self, commit: Commit);
    async fn get_commit(&self, id: &str) -> Option<Commit>;

    async fn link_commit_to_repository(&self, commit_id: &str, repo_id: &str);
    async fn link_commit_to_user(&self, commit_id: &str, user_id: &str);
    async fn get_commits_by_repository(&self, repo_id: &str) -> Vec<Commit>;
    async fn get_commits_by_user(&self, user_id: &str) -> Vec<Commit>;
}