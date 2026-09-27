use crate::service::{reset_service::ResetService, seed_service::SeedService};
use std::sync::Arc;

/// Application state shared across the orchestrator API.
#[derive(Debug, Clone)]
pub struct AppState {
    pub seed_service: Arc<SeedService>,
    pub reset_service: Arc<ResetService>,
}
