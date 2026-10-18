use crate::{
    error::AppError,
    model::seed_model::{HotspotSeed, UniformSeed, ZipfianSeed},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};

/// Handles the uniform seeding request.
pub async fn uniform_seed(
    State(state): State<AppState>,
    Json(payload): Json<UniformSeed>,
) -> Result<StatusCode, AppError> {
    state.seed_service.uniform(payload.n, payload.k).await?;
    Ok(StatusCode::OK)
}

/// Handles the zipfian seeding request.
pub async fn zipfian_seed(
    State(state): State<AppState>,
    Json(payload): Json<ZipfianSeed>,
) -> Result<StatusCode, AppError> {
    state.seed_service.zipfian(payload.n, payload.k).await?;
    Ok(StatusCode::OK)
}

/// Handles the hotspot seeding request.
pub async fn hotspot_seed(
    State(state): State<AppState>,
    Json(payload): Json<HotspotSeed>,
) -> Result<StatusCode, AppError> {
    state.seed_service.hotspot(payload.n, payload.k).await?;
    Ok(StatusCode::OK)
}
