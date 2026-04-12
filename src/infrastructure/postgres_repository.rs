use sqlx::PgPool;

use crate::{
    api::error::AppError,
    domain::{User, commit::Commit, repository::GraphRepository, repository_entity::Repository},
};

pub struct PostgresGraphRepository {
    pool: PgPool,
}

impl PostgresGraphRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl GraphRepository for PostgresGraphRepository {
    async fn create_user(&self, user: User) -> Result<(), AppError> {
        let result = sqlx::query("INSERT INTO users (id, username, email) VALUES ($1, $2, $3)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                // Handle duplicate key (important)
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.constraint().is_some() {
                        return Err(AppError::UserAlreadyExists);
                    }
                }
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn get_user(&self, id: &str) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, User>("SELECT id, username, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match row {
            Some(user) => Ok(user),
            None => Err(AppError::UserNotFound),
        }
    }

    async fn create_repository(&self, repo: Repository) -> Result<(), AppError> {
        let result = sqlx::query("INSERT INTO repositories (id, name) VALUES ($1, $2)")
            .bind(&repo.id)
            .bind(&repo.name)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.constraint().is_some() {
                        return Err(AppError::RepositoryAlreadyExists);
                    }
                }
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn get_repository(&self, id: &str) -> Result<Repository, AppError> {
        let row =
            sqlx::query_as::<_, Repository>("SELECT id, name FROM repositories WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match row {
            Some(repo) => Ok(repo),
            None => Err(AppError::RepositoryNotFound),
        }
    }

    async fn create_commit(&self, commit: Commit) -> Result<(), AppError> {
        let result = sqlx::query("INSERT INTO commits (id, message) VALUES ($1, $2)")
            .bind(&commit.id)
            .bind(&commit.message)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.constraint().is_some() {
                        return Err(AppError::CommitAlreadyExists);
                    }
                }
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn get_commit(&self, id: &str) -> Result<Commit, AppError> {
        let row = sqlx::query_as::<_, Commit>("SELECT id, message FROM commits WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match row {
            Some(commit) => Ok(commit),
            None => Err(AppError::CommitNotFound),
        }
    }

    async fn link_commit_to_repository(&self, _: &str, _: &str) -> Result<(), AppError> {
        unimplemented!()
    }

    async fn link_commit_to_user(&self, _: &str, _: &str) -> Result<(), AppError> {
        unimplemented!()
    }

    async fn get_commits_by_repository(&self, _: &str) -> Result<Vec<Commit>, AppError> {
        unimplemented!()
    }

    async fn get_commits_by_user(&self, _: &str) -> Result<Vec<Commit>, AppError> {
        unimplemented!()
    }
}
