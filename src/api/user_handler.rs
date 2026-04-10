use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;

use crate::{AppState, domain::{User, commit::Commit, repository::GraphRepository, repository_entity::Repository}};

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
) -> impl IntoResponse {
    let user = User::new(payload.id, payload.username, payload.email);
    state.repo.create_user(user).await;

    "User Created"
}

pub async fn create_repository(
    State(state) : State<AppState>,
    Json(payload): Json<CreateRepositoryRequest>
) -> impl IntoResponse {
    let repo = Repository::new(payload.id, payload.name);
    state.repo.create_repository(repo).await;

    "Repository Created"
}

pub async fn create_commit(
    State(state) : State<AppState>,
    Json(payload): Json<CreateCommitRequest>
) -> impl IntoResponse {
    let commit = Commit::new(payload.id, payload.message);
    state.repo.create_commit(commit).await;

    "Commit Created"
}

pub async fn link_commit_to_repository(
    State(state) : State<AppState>,
    Path((commit_id, repo_id)): Path<(String, String)>
) -> impl IntoResponse {
    state
        .repo
        .link_commit_to_repository(&commit_id, &repo_id)
        .await;

    "Commit linked to Repository"
}

pub async fn link_commit_to_user(
    State(state) : State<AppState>,
    Path((commit_id, user_id)): Path<(String, String)>
) -> impl IntoResponse {
    state
        .repo
        .link_commit_to_user(&commit_id, &user_id)
        .await;

    "Commit linked to User"
}

pub async fn get_commits_by_repository(
    State(state) : State<AppState>,
    Path((repo_id)): Path<String>
) -> Json<Vec<Commit>>{
    let commits = state.repo.get_commits_by_repository(&repo_id).await;

    Json(commits)
}

pub async fn get_commits_by_user(
    State(state) : State<AppState>,
    Path((user_id)): Path<String>
) -> Json<Vec<Commit>> {
    let commits = state.repo.get_commits_by_user(&user_id).await;

    Json(commits)
}