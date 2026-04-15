use std::time::Instant;

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{
    api::error::AppError,
    domain::{User, commit::Commit, repository::GraphRepository, repository_entity::Repository},
    infrastructure::metrics::{record_db_error, record_db_query},
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
        let start = Instant::now();

        let result = sqlx::query("SELECT 1").execute(&self.pool).await;

        match result {
            Ok(_) => {
                record_db_query("db_health", start);
                Ok(())
            }
            Err(e) => {
                record_db_error("db_health");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn create_user(&self, user: User) -> Result<(), AppError> {
        let start = Instant::now();
        let result = sqlx::query("INSERT INTO users (id, username, email) VALUES ($1, $2, $3)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                record_db_query("create_user", start);
                Ok(())
            }
            Err(e) => {
                record_db_error("create_user");
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
        let start = Instant::now();
        let row = sqlx::query_as::<_, User>("SELECT id, username, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match row {
            Some(user) => {
                record_db_query("get_user", start);
                Ok(user)
            }
            None => {
                record_db_error("get_user");
                Err(AppError::UserNotFound)
            }
        }
    }

    async fn create_repository(&self, repo: Repository) -> Result<(), AppError> {
        let start = Instant::now();
        let result = sqlx::query("INSERT INTO repositories (id, name) VALUES ($1, $2)")
            .bind(&repo.id)
            .bind(&repo.name)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                record_db_query("create_repository", start);
                Ok(())
            }
            Err(e) => {
                record_db_error("create_repository");
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
        let start = Instant::now();
        let row =
            sqlx::query_as::<_, Repository>("SELECT id, name FROM repositories WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match row {
            Some(repo) => {
                record_db_query("get_repository", start);
                Ok(repo)
            }
            None => {
                record_db_error("get_repository");
                Err(AppError::RepositoryNotFound)
            }
        }
    }

    async fn create_commit(&self, commit: Commit) -> Result<(), AppError> {
        let start = Instant::now();
        let result = sqlx::query("INSERT INTO commits (id, message) VALUES ($1, $2)")
            .bind(&commit.id)
            .bind(&commit.message)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                record_db_query("create_commit", start);
                Ok(())
            }
            Err(e) => {
                record_db_error("create_commit");
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
        let start = Instant::now();
        let row = sqlx::query_as::<_, Commit>("SELECT id, message FROM commits WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match row {
            Some(commit) => {
                record_db_query("get_commit", start);
                Ok(commit)
            }
            None => {
                record_db_error("get_commit");
                Err(AppError::CommitNotFound)
            }
        }
    }

    async fn link_commit_to_repository(
        &self,
        commit_id: &str,
        repo_id: &str,
    ) -> Result<(), AppError> {
        let start = Instant::now();
        let result =
            sqlx::query("INSERT INTO commit_repository (commit_id, repo_id) VALUES ($1, $2)")
                .bind(commit_id)
                .bind(repo_id)
                .execute(&self.pool)
                .await;

        match result {
            Ok(_) => {
                record_db_query("link_commit_to_repository", start);
                Ok(())
            }

            Err(e) => {
                record_db_error("link_commit_to_repository");
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
        let start = Instant::now();
        let result = sqlx::query("INSERT INTO commit_user (commit_id, user_id) VALUES ($1, $2)")
            .bind(commit_id)
            .bind(user_id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                record_db_query("link_commit_to_user", start);
                Ok(())
            }

            Err(e) => {
                record_db_error("link_commit_to_user");
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

    async fn get_commits_by_repository(
        &self,
        repo_id: &str,
        limit: u32,
        cursor: Option<String>,
    ) -> Result<Vec<Commit>, AppError> {
        let start = Instant::now();

        // 1. Validate repository exists
        // We record this as a "query" because the DB successfully answered us,
        // even if it says the repo doesn't exist.
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)",
        )
        .bind(repo_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            record_db_error("repo_exists_check");
            AppError::InternalServerError(e.to_string())
        })?;

        if !exists {
            return Err(AppError::RepositoryNotFound);
        }

        // 2. Execute the fetch
        let result = if let Some(cursor) = cursor {
            sqlx::query_as::<_, Commit>(
                r#"
                SELECT c.id, c.message
                FROM commit_repository cr
                JOIN commits c ON c.id = cr.commit_id
                WHERE cr.repo_id = $1 AND cr.commit_id > $2
                ORDER BY cr.commit_id ASC
                LIMIT $3
                "#,
            )
            .bind(repo_id)
            .bind(cursor)
            .bind(limit as i32)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, Commit>(
                r#"
                SELECT c.id, c.message
                FROM commit_repository cr
                JOIN commits c ON c.id = cr.commit_id
                WHERE cr.repo_id = $1
                ORDER BY cr.commit_id ASC
                LIMIT $2
                "#,
            )
            .bind(repo_id)
            .bind(limit as i32)
            .fetch_all(&self.pool)
            .await
        };

        // 3. Final Metric Recording (The "Match" Pattern)
        match result {
            Ok(commits) => {
                record_db_query("get_commits_by_repository", start);
                Ok(commits)
            }
            Err(e) => {
                record_db_error("get_commits_by_repository");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn get_commits_by_user(
        &self,
        user_id: &str,
        limit: u32,
        cursor: Option<String>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<Vec<Commit>, AppError> {
        use crate::infrastructure::metrics::{record_db_error, record_db_query};
        use std::time::Instant;

        let start = Instant::now();

        // Validate user exists
        let exists = sqlx::query_scalar::<_, i32>("SELECT 1 FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await;

        if let Err(e) = exists {
            record_db_error("get_commits_by_user");
            return Err(AppError::InternalServerError(e.to_string()));
        }

        if exists.unwrap().is_none() {
            return Err(AppError::UserNotFound);
        }

        // Build query dynamically
        let mut query = String::from(
            r#"
        SELECT c.id, c.message
        FROM commit_user cu
        JOIN commits c ON c.id = cu.commit_id
        WHERE cu.user_id = $1
        "#,
        );

        let mut bind_index = 2;

        if cursor.is_some() {
            query.push_str(&format!(" AND cu.commit_id > ${}", bind_index));
            bind_index += 1;
        }

        if from.is_some() {
            query.push_str(&format!(" AND c.created_at >= ${}", bind_index));
            bind_index += 1;
        }

        if to.is_some() {
            query.push_str(&format!(" AND c.created_at <= ${}", bind_index));
            bind_index += 1;
        }

        query.push_str(&format!(" ORDER BY cu.commit_id LIMIT ${}", bind_index));

        let mut q = sqlx::query_as::<_, Commit>(&query).bind(user_id);

        if let Some(cursor) = cursor {
            q = q.bind(cursor);
        }

        if let Some(from) = from {
            q = q.bind(from);
        }

        if let Some(to) = to {
            q = q.bind(to);
        }

        q = q.bind(limit as i32);

        let result = q.fetch_all(&self.pool).await;

        match result {
            Ok(commits) => {
                record_db_query("get_commits_by_user", start);
                Ok(commits)
            }
            Err(e) => {
                record_db_error("get_commits_by_user");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    async fn get_repositories_by_user(
        &self,
        user_id: &str,
        limit: u32,
        cursor: Option<String>,
    ) -> Result<Vec<Repository>, AppError> {
        use crate::infrastructure::metrics::{record_db_error, record_db_query};
        use std::time::Instant;

        let start = Instant::now();

        // Validate user exists
        let exists = sqlx::query_scalar::<_, i32>("SELECT 1 FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await;

        if let Err(e) = exists {
            record_db_error("get_repositories_by_user");
            return Err(AppError::InternalServerError(e.to_string()));
        }

        if exists.unwrap().is_none() {
            return Err(AppError::UserNotFound);
        }

        let result = if let Some(cursor) = cursor {
            sqlx::query_as::<_, Repository>(
                r#"
            SELECT DISTINCT r.id, r.name
            FROM commit_user cu
            JOIN commits c ON c.id = cu.commit_id
            JOIN commit_repository cr ON cr.commit_id = c.id
            JOIN repositories r ON r.id = cr.repo_id
            WHERE cu.user_id = $1
              AND r.id > $2
            ORDER BY r.id
            LIMIT $3
            "#,
            )
            .bind(user_id)
            .bind(cursor)
            .bind(limit as i32)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, Repository>(
                r#"
            SELECT DISTINCT r.id, r.name
            FROM commit_user cu
            JOIN commits c ON c.id = cu.commit_id
            JOIN commit_repository cr ON cr.commit_id = c.id
            JOIN repositories r ON r.id = cr.repo_id
            WHERE cu.user_id = $1
            ORDER BY r.id
            LIMIT $2
            "#,
            )
            .bind(user_id)
            .bind(limit as i32)
            .fetch_all(&self.pool)
            .await
        };

        match result {
            Ok(repos) => {
                record_db_query("get_repositories_by_user", start);
                Ok(repos)
            }
            Err(e) => {
                record_db_error("get_repositories_by_user");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }
}
