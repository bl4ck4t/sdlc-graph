use axum::{Json, http::{StatusCode}, response::{IntoResponse, Response}};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User Not Found")]
    UserNotFound,

    #[error("Repository Not Found")]
    RepositoryNotFound,

    #[error("Commit Not Found")]
    CommitNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Repository already exists")]
    RepositoryAlreadyExists,

    #[error("Commit already exists")]
    CommitAlreadyExists,

    #[error("Internal Server Error")]
    InternalServerError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::UserNotFound => StatusCode::NOT_FOUND,
            AppError::RepositoryNotFound => StatusCode::NOT_FOUND,
            AppError::CommitNotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserAlreadyExists => StatusCode::CONFLICT,
            AppError::RepositoryAlreadyExists => StatusCode::CONFLICT,
            AppError::CommitAlreadyExists => StatusCode::CONFLICT,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(ErrorResponse {
            message: self.to_string()
        });

        (status, body).into_response()
    }
}