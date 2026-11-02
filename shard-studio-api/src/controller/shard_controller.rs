use crate::{error::AppError, model::shard_model::ShardRequest, state::AppState};
use axum::{Json, extract::State, http::StatusCode};

/// Handles the sharding request.
pub async fn shard(
    State(state): State<AppState>,
    Json(payload): Json<ShardRequest>,
) -> Result<StatusCode, AppError> {
    state.shard_service.shard(payload).await?;
    Ok(StatusCode::OK)
}
