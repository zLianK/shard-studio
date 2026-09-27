use crate::{error::AppError, state::AppState};
use axum::{extract::State, http::StatusCode};

/// Handles the simulation reset request.
pub async fn reset(State(state): State<AppState>) -> Result<StatusCode, AppError> {
    state.reset_service.reset().await?;
    Ok(StatusCode::OK)
}
