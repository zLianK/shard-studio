use crate::{
    controller::{
        reset_controller::reset,
        seed_controller::{hotspot_seed, uniform_seed, zipfian_seed},
    },
    state::AppState,
};
use axum::{Router, routing::post};

/// Configures the application routes for the Shard Studio API.
pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/seed/uniform", post(uniform_seed))
        .route("/seed/zipfian", post(zipfian_seed))
        .route("/seed/hotspot", post(hotspot_seed))
        .route("/reset", post(reset))
}
