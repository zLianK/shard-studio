use crate::{
    controller::{
        report_controller::{get_report, save_report},
        reset_controller::reset,
        seed_controller::seed,
        shard_controller::shard,
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
        .route("/shard", post(shard))
        .route("/reset", post(reset))
        .route("/report", get(get_report).post(save_report))
}
