mod api;
mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::{TcpListener};

use axum::{Router, extract::State, routing::{get, post}};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{api::user_handler::{create_commit, create_repository, create_user, get_commit, get_commits_by_repository, get_commits_by_user, get_repository, get_user, link_commit_to_repository, link_commit_to_user}, application::services::graph_service::GraphService, domain::repository::GraphRepository, infrastructure::{in_memory_repository::InMemoryGraphRepository, postgres_repository::PostgresGraphRepository}};

#[derive(Clone)]
struct AppState {
    service: Arc<GraphService>,
    db: PgPool
}

#[tokio::main]
async fn main() {

    let database_url = "postgres://postgres:toor@localhost:5432/graph_db";

    let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(database_url)
    .await
    .expect("Failed to connect to Postgres");

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let repo = Arc::new(PostgresGraphRepository::new(db.clone()));
    let service = Arc::new(GraphService::new(repo));

    let state = AppState { service, db };

    tracing::info!("Connected to Postgres");

    let app = Router::new()
        .route("/", get(root))
        .route("/health/db", get(db_health))
        
        //Users
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id/commits", get(get_commits_by_user))
        
        //Repositories
        .route("/repos", post(create_repository))
        .route("/repos/:id", get(get_repository))
        .route("/repos/:id/commits", get(get_commits_by_repository))
        
        //Commits
        .route("/commits", post(create_commit))
        .route("/commits/:id", get(get_commit))
        
        //edges
        .route(
            "/commits/:commit_id/link-repo/:repo_id",
            post(link_commit_to_repository)
        )
        
        .route(
            "/commits/:commit_id/link-user/:user_id",
            post(link_commit_to_user)    
        )
        
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
    DefaultMakeSpan::new()
                    .include_headers(false)
                    .level(Level::INFO) // This triggers the "started" log
            )
            .on_response(
DefaultOnResponse::new()
                    .level(Level::INFO) // This triggers the "completed" log
                    .latency_unit(tower_http::LatencyUnit::Millis)
            ),
        )
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

async fn db_health(State(state): State<AppState>) -> &'static str {
    match sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
    {
        Ok(_) => "DB OK",
        Err(_) => "DB DOWN",
    }
}
