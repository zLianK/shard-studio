use crate::{
    error::AppError,
    model::seed_model::{UniformSeed, ZipfianSeed},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};

/// Handles the uniform seeding request.
pub async fn uniform_seed(
    State(state): State<AppState>,
    Json(payload): Json<UniformSeed>,
) -> Result<StatusCode, AppError> {
    state.seed_service.uniform(payload.n).await?;
    Ok(StatusCode::OK)
}

/// Handles the zipfian seeding request.
pub async fn zipfian_seed(
    State(state): State<AppState>,
    Json(payload): Json<ZipfianSeed>,
) -> Result<StatusCode, AppError> {
    state.seed_service.zipfian(payload.n, payload.s).await?;
    Ok(StatusCode::OK)
}
