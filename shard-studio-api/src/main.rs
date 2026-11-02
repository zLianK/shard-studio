use crate::{
    log::log_install,
    routes::app_routes,
    service::{
        cluster_service::ClusterService, report_service::ReportService,
        reset_service::ResetService, seed_service::SeedService, shard_service::ShardService,
    },
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
mod sharding;
mod state;

#[tokio::main]
async fn main() {
    log_install();

    let cluster_service = Arc::new(ClusterService::new());
    let report_service = Arc::new(ReportService::new());
    let seed_service = Arc::new(SeedService::new(
        cluster_service.clone(),
        report_service.clone(),
    ));
    let shard_service = Arc::new(ShardService::new(cluster_service.clone()));
    let reset_service = Arc::new(ResetService::new(
        cluster_service.clone(),
        report_service.clone(),
    ));

    let state = AppState {
        seed_service,
        shard_service,
        reset_service,
        report_service,
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
