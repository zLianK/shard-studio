use crate::{
    controller::{reset_controller::reset, seed_controller::seed},
    state::AppState,
};
use axum::{Router, routing::post};

/// Configures the application routes for the Shard Studio API.
pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/seed", post(seed))
        .route("/reset", post(reset))
}
