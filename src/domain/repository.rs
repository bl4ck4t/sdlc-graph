use crate::domain::User;

#[async_trait::async_trait]
pub trait GraphRepository: Send + Sync {
    async fn create_user(&self, user: User);

    async fn get_user(&self, id: &str) -> Option<User>;
}