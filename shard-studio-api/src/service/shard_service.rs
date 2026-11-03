use crate::{
    error::{AppError, AppResult},
    generator::DataObject,
    model::{
        report_model::ShardingReport,
        shard_model::{ShardRequest, ShardingAlgorithm},
    },
    repository::postgres::PostgresRepository,
    service::{cluster_service::ClusterService, report_service::ReportService},
    sharding::{hash::HashStrategy, strategy::ShardingStrategy},
};
use error_stack::bail;
use std::sync::Arc;
use tracing::info;

/// Maximum number of shards, as each one is a container.
const MAX_SHARDS: u64 = 16;

/// Number of rows read from the main database at a time.
const BATCH_SIZE: i64 = 1000;

/// Service responsible for handling sharding operations.
#[derive(Debug, Clone)]
pub struct ShardService {
    cluster_service: Arc<ClusterService>,
    report_service: Arc<ReportService>,
}

impl ShardService {
    pub fn new(cluster_service: Arc<ClusterService>, report_service: Arc<ReportService>) -> Self {
        Self {
            cluster_service,
            report_service,
        }
    }

    /// Handles the sharding request. Distributes the rows of the main
    /// database across the shards following the requested strategy.
    pub async fn shard(&self, request: ShardRequest) -> AppResult<()> {
        let strategy = build_strategy(&request)?;
        let main = self.cluster_service.main().await?;
        let shards = self
            .cluster_service
            .start_shards(request.shards as usize)
            .await?;
        info!(
            "Sharding with {:?} strategy over {} shards",
            request.strategy, request.shards
        );

        let moved = distribute_rows(&main, &shards, strategy.as_ref()).await?;
        info!("Moved {} data objects to {} shards", moved, shards.len());

        self.report_service
            .set_sharding(ShardingReport::new(&request))
            .await;
        Ok(())
    }
}

/// Reads every row of the main database, in batches, and writes each one
/// to the shard chosen by the strategy. Returns the number of rows moved.
async fn distribute_rows(
    main: &PostgresRepository,
    shards: &[PostgresRepository],
    strategy: &dyn ShardingStrategy,
) -> AppResult<u64> {
    let mut moved = 0;
    let mut last_row_id = 0;
    loop {
        let batch = main.fetch_batch(last_row_id, BATCH_SIZE).await?;
        let Some((row_id, _)) = batch.last() else {
            return Ok(moved);
        };
        last_row_id = *row_id;
        moved += batch.len() as u64;

        let mut per_shard: Vec<Vec<DataObject>> = vec![Vec::new(); shards.len()];
        for (_, object) in batch {
            per_shard[strategy.shard_for(object.id)].push(object);
        }
        for (shard, objects) in shards.iter().zip(&per_shard) {
            shard.save_all(objects).await?;
        }
    }
}

/// Builds the sharding strategy described by the request.
fn build_strategy(request: &ShardRequest) -> AppResult<Box<dyn ShardingStrategy>> {
    if request.shards > MAX_SHARDS {
        bail!(AppError::ShardingError(format!(
            "the number of shards must be at most {MAX_SHARDS}"
        )));
    }

    Ok(match request.strategy {
        ShardingAlgorithm::Hash => Box::new(HashStrategy::new(request.shards)?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(shards: u64) -> ShardRequest {
        ShardRequest {
            strategy: ShardingAlgorithm::Hash,
            shards,
        }
    }

    #[test]
    fn number_of_shards_must_be_within_limits() {
        assert!(build_strategy(&request(1)).is_err());
        assert!(build_strategy(&request(2)).is_ok());
        assert!(build_strategy(&request(MAX_SHARDS)).is_ok());
        assert!(build_strategy(&request(MAX_SHARDS + 1)).is_err());
    }
}
