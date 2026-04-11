use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;

use crate::{AppState, api::error::AppError, domain::{User, commit::Commit, repository::GraphRepository, repository_entity::Repository}};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    id: String,
    username: String,
    email: String
}

#[derive(Deserialize)]
pub struct CreateRepositoryRequest {
    pub id: String,
    pub name: String
}

#[derive(Deserialize)]
pub struct CreateCommitRequest {
    pub id: String,
    pub message: String
}

pub async fn create_user(
    State(state) : State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Result<Json<User>, AppError> {
    let user = User::new(payload.id, payload.username, payload.email);
    state.repo.create_user(user.clone()).await?;

    Ok(Json(user))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<Json<User>, AppError> {
    let user = state.repo.get_user(&id).await?;
    Ok(Json(user))
}

pub async fn create_repository(
    State(state) : State<AppState>,
    Json(payload): Json<CreateRepositoryRequest>
) -> Result<Json<Repository>, AppError> {
    let repo = Repository::new(payload.id, payload.name);
    state.repo.create_repository(repo.clone()).await;

    Ok(Json(repo))
}

pub async fn create_commit(
    State(state) : State<AppState>,
    Json(payload): Json<CreateCommitRequest>
) -> Result<Json<Commit>, AppError> {
    let commit = Commit::new(payload.id, payload.message);
    state.repo.create_commit(commit.clone()).await;

    Ok(Json(commit))
}

pub async fn get_repository(
    State(state) : State<AppState>,
    Path(id): Path<String>
) -> Result<Json<Repository>, AppError> {
    let repo = state.repo.get_repository(&id).await?;
    Ok(Json(repo))
}

pub async fn get_commit(
    State(state) : State<AppState>,
    Path(id): Path<String>
) -> Result<Json<Commit>, AppError> {
    let commit = state.repo.get_commit(&id).await?;
    Ok(Json(commit))
}

pub async fn link_commit_to_repository(
    State(state) : State<AppState>,
    Path((commit_id, repo_id)): Path<(String, String)>
) -> Result<&'static str, AppError> {
    state
        .repo
        .link_commit_to_repository(&commit_id, &repo_id)
        .await?;

    Ok("Commit linked to Repository")
}

pub async fn link_commit_to_user(
    State(state) : State<AppState>,
    Path((commit_id, user_id)): Path<(String, String)>
) -> Result<&'static str, AppError> {
    state
        .repo
        .link_commit_to_user(&commit_id, &user_id)
        .await?;

    Ok("Commit linked to User")
}

pub async fn get_commits_by_repository(
    State(state) : State<AppState>,
    Path((repo_id)): Path<String>
) -> Result<Json<Vec<Commit>>, AppError> {
    let commits = state.repo.get_commits_by_repository(&repo_id).await?;

    Ok(Json(commits))
}

pub async fn get_commits_by_user(
    State(state) : State<AppState>,
    Path((user_id)): Path<String>
) -> Result<Json<Vec<Commit>>, AppError> {
    let commits = state.repo.get_commits_by_user(&user_id).await?;

    Ok(Json(commits))
}