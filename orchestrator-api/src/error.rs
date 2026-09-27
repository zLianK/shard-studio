use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error_stack::Report;
use serde_json::json;
use thiserror::Error;

/// Application-specific error type.
#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("error occurred in seed distribution: {0}")]
    SeedDistributionError(String),
    #[error("docker operation failed: {0}")]
    DockerError(String),
}

/// Type alias for application results.
pub type AppResult<T> = Result<T, Report<AppError>>;

impl From<Report<AppError>> for AppError {
    fn from(report: Report<AppError>) -> Self {
        report.current_context().clone()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::SeedDistributionError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DockerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
