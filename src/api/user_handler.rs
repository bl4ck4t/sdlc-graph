use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use tracing::{info, instrument};

use crate::{
    AppState,
    api::error::AppError,
    domain::{User, commit::Commit, repository_entity::Repository},
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    id: String,
    username: String,
    email: String,
}

#[derive(Deserialize)]
pub struct CreateRepositoryRequest {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateCommitRequest {
    pub id: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct CursorPagination {
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

fn validate_non_empty(field: &str, value: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "{} cannot be empty",
            field
        )));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), AppError> {
    if !email.contains('@') {
        return Err(AppError::ValidationError(
            "email must contain '@'".to_string(),
        ));
    }
    Ok(())
}

pub async fn db_health(State(state): State<AppState>) -> &'static str {
    match state.service.db_health().await {
        Ok(_) => "DB_OK",
        Err(_) => "DB_DOWN",
    }
}

#[instrument(skip(state))]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    info!("creating user with id={}", payload.id);

    validate_non_empty("id", &payload.id)?;
    validate_non_empty("username", &payload.username)?;
    validate_non_empty("email", &payload.email)?;
    validate_email(&payload.email)?;

    let user = User::new(payload.id, payload.username, payload.email);
    state.service.create_user(user.clone()).await?;

    Ok(Json(user))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>, AppError> {
    info!("fetching user with id={}", id);

    let user = state.service.get_user(&id).await?;
    Ok(Json(user))
}

pub async fn create_repository(
    State(state): State<AppState>,
    Json(payload): Json<CreateRepositoryRequest>,
) -> Result<Json<Repository>, AppError> {
    info!("creating repository with id={}", payload.id);

    validate_non_empty("id", &payload.id)?;
    validate_non_empty("name", &payload.name)?;

    let repo = Repository::new(payload.id, payload.name);
    state.service.create_repository(repo.clone()).await?;

    Ok(Json(repo))
}

pub async fn create_commit(
    State(state): State<AppState>,
    Json(payload): Json<CreateCommitRequest>,
) -> Result<Json<Commit>, AppError> {
    info!("creating commit with id={}", payload.id);

    validate_non_empty("id", &payload.id)?;
    validate_non_empty("message", &payload.message)?;

    let commit = Commit::new(payload.id, payload.message);
    state.service.create_commit(commit.clone()).await?;

    Ok(Json(commit))
}

pub async fn get_repository(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Repository>, AppError> {
    info!("fetching repository with id={}", id);

    let repo = state.service.get_repository(&id).await?;
    Ok(Json(repo))
}

pub async fn get_commit(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Commit>, AppError> {
    info!("fetching commit with id={}", id);

    let commit = state.service.get_commit(&id).await?;
    Ok(Json(commit))
}

pub async fn link_commit_to_repository(
    State(state): State<AppState>,
    Path((commit_id, repo_id)): Path<(String, String)>,
) -> Result<&'static str, AppError> {
    info!("linking commit {} to repository {}", commit_id, repo_id);

    state
        .service
        .link_commit_to_repository(&commit_id, &repo_id)
        .await?;

    Ok("Commit linked to Repository")
}

pub async fn link_commit_to_user(
    State(state): State<AppState>,
    Path((commit_id, user_id)): Path<(String, String)>,
) -> Result<&'static str, AppError> {
    info!("linking commit {} to user {}", commit_id, user_id);

    state
        .service
        .link_commit_to_user(&commit_id, &user_id)
        .await?;

    Ok("Commit linked to User")
}

pub async fn get_commits_by_repository(
    State(state): State<AppState>,
    Path(repo_id): Path<String>,
    Query(pagination): Query<CursorPagination>,
) -> Result<Json<Vec<Commit>>, AppError> {
    let limit = pagination.limit.unwrap_or(10).min(100);
    let cursor = pagination.cursor;

    info!("fetching commits for repo {}", repo_id);

    let commits = state
        .service
        .get_commits_by_repository(&repo_id, limit, cursor)
        .await?;

    Ok(Json(commits))
}

pub async fn get_commits_by_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(pagination): Query<CursorPagination>,
) -> Result<Json<Vec<Commit>>, AppError> {
    let limit = pagination.limit.unwrap_or(10).min(100);
    let cursor = pagination.cursor;

    info!("fetching commits for user {}", user_id);

    let commits = state
        .service
        .get_commits_by_user(&user_id, limit, cursor)
        .await?;

    Ok(Json(commits))
}

pub async fn get_user_repositories(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(pagination): Query<CursorPagination>,
) -> Result<Json<Vec<Repository>>, AppError> {
    let limit = pagination.limit.unwrap_or(10);

    let repos = state
        .service
        .get_repositories_by_user(&user_id, limit, pagination.cursor)
        .await?;

    Ok(Json(repos))
}
