mod domain;
mod application;
mod infrastructure;

use tokio::net::{TcpListener};

use axum::{Router, routing::get};

#[tokio::main]

async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

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