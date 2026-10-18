use crate::{error::AppError, model::seed_model::SeedRequest, state::AppState};
use axum::{Json, extract::State, http::StatusCode};

/// Handles the seeding request.
pub async fn seed(
    State(state): State<AppState>,
    Json(payload): Json<SeedRequest>,
) -> Result<StatusCode, AppError> {
    state.seed_service.seed(payload).await?;
    Ok(StatusCode::OK)
}
