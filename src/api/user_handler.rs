use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::{AppState, domain::{User, repository::GraphRepository}};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    id: String,
    username: String,
    email: String
}

pub async fn create_user(
    State(state) : State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> impl IntoResponse {
    let user = User::new(payload.id, payload.username, payload.email);
    state.repo.create_user(user).await;

    "User Created"
}