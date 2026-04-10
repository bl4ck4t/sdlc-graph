use crate::domain::{User, commit::Commit, repository_entity::Repository};

#[async_trait::async_trait]
pub trait GraphRepository: Send + Sync {
    async fn create_user(&self, user: User);
    async fn get_user(&self, id: &str) -> Option<User>;

    async fn create_repository(&self, repo: Repository);
    async fn get_repository(&self, id: &str) -> Option<Repository>;

    async fn create_commit(&self, commit: Commit);
    async fn get_commit(&self, id: &str) -> Option<Commit>;
}