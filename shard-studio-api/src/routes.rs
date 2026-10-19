use crate::{
    controller::{
        report_controller::{get_report, save_report},
        reset_controller::reset,
        seed_controller::seed,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

/// Configures the application routes for the Shard Studio API.
pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/seed", post(seed))
        .route("/reset", post(reset))
        .route("/report", get(get_report).post(save_report))
}
