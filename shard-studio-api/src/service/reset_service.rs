use crate::{
    docker::{remove_containers_with_prefix, utils::container_name_prefix},
    error::AppResult,
};

/// Service responsible for resetting the simulation.
#[derive(Debug, Clone, Default)]
pub struct ResetService;

impl ResetService {
    /// Resets the simulation by removing all containers created by the application.
    pub async fn reset(&self) -> AppResult<()> {
        let prefix = container_name_prefix();
        remove_containers_with_prefix(&prefix).await
    }
}
