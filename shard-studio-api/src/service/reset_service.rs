use crate::{
    docker::{remove_containers_with_prefix, utils::container_name_prefix},
    error::AppResult,
    service::report_service::ReportService,
};
use std::sync::Arc;

/// Service responsible for resetting the simulation.
#[derive(Debug, Clone)]
pub struct ResetService {
    report_service: Arc<ReportService>,
}

impl ResetService {
    pub fn new(report_service: Arc<ReportService>) -> Self {
        Self { report_service }
    }

    /// Resets the simulation by starting a new experiment report and
    /// removing all containers created by the application.
    pub async fn reset(&self) -> AppResult<()> {
        let prefix = container_name_prefix();
        self.report_service.reset().await;
        remove_containers_with_prefix(&prefix).await
    }
}
