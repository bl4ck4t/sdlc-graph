mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;

use tokio::net::{TcpListener};

use axum::{Router, extract::State, routing::get};

use crate::{domain::repository::GraphRepository, infrastructure::in_memory_repository::InMemoryGraphRepository};

#[derive(Clone)]
struct AppState {
    repo: Arc<InMemoryGraphRepository>
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let repo = Arc::new(InMemoryGraphRepository::new());

    let state = AppState { repo };

    let app = Router::new()
        .route("/", get(root))
        .route("/users/:id", get(get_user))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    //Use following command to view logs in powershell
    //$env:RUST_LOG="info"; cargo run 
    tracing::info!("Server running on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap()
}

async fn root() -> &'static str {
    "Hello, SDLC Graph!"
}

async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>
) -> String {
    let user = state.repo.get_user(&id).await;

    match user {
        Some(u) => format!("User found: {}", u.username),
        None => "User not found".to_string(),
    }
}