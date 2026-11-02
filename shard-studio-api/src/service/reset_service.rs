use crate::{
    error::AppResult,
    service::{cluster_service::ClusterService, report_service::ReportService},
};
use std::sync::Arc;

/// Service responsible for resetting the simulation.
#[derive(Debug, Clone)]
pub struct ResetService {
    cluster_service: Arc<ClusterService>,
    report_service: Arc<ReportService>,
}

impl ResetService {
    pub fn new(cluster_service: Arc<ClusterService>, report_service: Arc<ReportService>) -> Self {
        Self {
            cluster_service,
            report_service,
        }
    }

    /// Resets the simulation by starting a new experiment report and
    /// resetting the cluster.
    pub async fn reset(&self) -> AppResult<()> {
        self.report_service.reset().await;
        self.cluster_service.reset().await
    }
}
