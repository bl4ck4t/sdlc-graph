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

    async fn db_health(&self) -> Result<(), AppError> {
        sqlx::query("SELECT 1").execute(&self.pool).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

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

    async fn link_commit_to_repository(
        &self,
        commit_id: &str,
        repo_id: &str,
    ) -> Result<(), AppError> {
        let result =
            sqlx::query("INSERT INTO commit_repository (commit_id, repo_id) VALUES ($1, $2)")
                .bind(commit_id)
                .bind(repo_id)
                .execute(&self.pool)
                .await;

        match result {
            Ok(_) => Ok(()),

            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    if let Some(constraint) = db_err.constraint() {
                        if constraint.contains("commit_repository_commit_id_fkey") {
                            return Err(AppError::CommitNotFound);
                        }

                        if constraint.contains("commit_repository_repo_id_fkey") {
                            return Err(AppError::RepositoryNotFound);
                        }

                        if constraint.contains("commit_repository_pkey") {
                            return Err(AppError::ValidationError(
                                "commit already linked to repository".to_string(),
                            ));
                        }
                    }
                }

                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn link_commit_to_user(&self, commit_id: &str, user_id: &str) -> Result<(), AppError> {
        let result = sqlx::query("INSERT INTO commit_user (commit_id, user_id) VALUES ($1, $2)")
            .bind(commit_id)
            .bind(user_id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),

            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    // Foreign key violations
                    if let Some(constraint) = db_err.constraint() {
                        if constraint.contains("commit_user_commit_id_fkey") {
                            return Err(AppError::CommitNotFound);
                        }

                        if constraint.contains("commit_user_user_id_fkey") {
                            return Err(AppError::UserNotFound);
                        }

                        // Primary key violation (duplicate link)
                        if constraint.contains("commit_user_pkey") {
                            return Err(AppError::ValidationError(
                                "commit already linked to user".to_string(),
                            ));
                        }
                    }
                }

                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn get_commits_by_repository(&self, repo_id: &str) -> Result<Vec<Commit>, AppError> {
        // Step 1: validate repo exists
        let exists =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM repositories WHERE id = $1")
                .bind(repo_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if exists == 0 {
            return Err(AppError::RepositoryNotFound);
        }

        // Step 2: fetch commits
        let commits = sqlx::query_as::<_, Commit>(
            r#"
        SELECT c.id, c.message
        FROM commits c
        JOIN commit_repository cr ON c.id = cr.commit_id
        WHERE cr.repo_id = $1
        "#,
        )
        .bind(repo_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(commits)
    }

    async fn get_commits_by_user(&self, user_id: &str) -> Result<Vec<Commit>, AppError> {
        // Step 1: validate user exists
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if exists == 0 {
            return Err(AppError::UserNotFound);
        }

        // Step 2: fetch commits
        let commits = sqlx::query_as::<_, Commit>(
            r#"
        SELECT c.id, c.message
        FROM commits c
        JOIN commit_user cu ON c.id = cu.commit_id
        WHERE cu.user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(commits)
    }
}
