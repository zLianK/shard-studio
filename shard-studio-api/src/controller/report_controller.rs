use crate::{error::AppError, model::report_model::Report, state::AppState};
use axum::{Json, extract::State};
use serde_json::{Value, json};

/// Saves the report of the current experiment to disk.
/// Responds with the name of the saved report.
pub async fn save_report(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let name = state.report_service.save_current().await?;
    Ok(Json(json!({ "name": name })))
}

/// Returns the report of the current experiment.
pub async fn get_report(State(state): State<AppState>) -> Json<Report> {
    Json(state.report_service.current().await)
}
