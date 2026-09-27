use crate::{
    controller::{
        reset_controller::reset,
        seed_controller::{uniform_seed, zipfian_seed},
    },
    state::AppState,
};
use axum::{Router, routing::post};

/// Configures the application routes for the orchestrator API.
pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/seed/uniform", post(uniform_seed))
        .route("/seed/zipfian", post(zipfian_seed))
        .route("/reset", post(reset))
}
