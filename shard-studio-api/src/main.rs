use crate::{
    log::log_install,
    routes::app_routes,
    service::{reset_service::ResetService, seed_service::SeedService},
    state::AppState,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

mod controller;
mod docker;
mod error;
mod generator;
mod log;
mod model;
mod repository;
mod routes;
mod service;
mod state;

#[tokio::main]
async fn main() {
    log_install();

    let seed_service = Arc::new(SeedService);
    let reset_service = Arc::new(ResetService);

    let state = AppState {
        seed_service,
        reset_service,
    };
    let app = app_routes().with_state(state);

    let listener = get_listener().await;
    axum::serve(listener, app).await.unwrap();
}

/// Returns a TCP listener for the Shard Studio API.
async fn get_listener() -> TcpListener {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("listening on {addr}");
    listener
}
