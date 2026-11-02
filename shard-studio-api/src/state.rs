use crate::service::{
    report_service::ReportService, reset_service::ResetService, seed_service::SeedService,
    shard_service::ShardService,
};
use std::sync::Arc;

/// Application state shared across the Shard Studio API.
#[derive(Debug, Clone)]
pub struct AppState {
    pub seed_service: Arc<SeedService>,
    pub shard_service: Arc<ShardService>,
    pub reset_service: Arc<ResetService>,
    pub report_service: Arc<ReportService>,
}
