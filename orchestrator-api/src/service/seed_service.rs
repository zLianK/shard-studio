use crate::{
    docker::{create_and_start_container, postgres::PostgresContainer},
    error::AppResult,
    generator::{
        DataGenerator, strategy::DistributionStrategy, uniform::UniformStrategy,
        zipfian::ZipfianStrategy,
    },
};
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Service responsible for handling seeding operations.
#[derive(Debug, Clone, Default)]
pub struct SeedService;

impl SeedService {
    /// Handles the uniform seeding request.
    pub async fn uniform(&self, n: u64) -> AppResult<()> {
        data_seeding(n, UniformStrategy::new(n)?).await
    }

    /// Handles the zipfian seeding request.
    pub async fn zipfian(&self, n: u64, s: f64) -> AppResult<()> {
        data_seeding(n, ZipfianStrategy::new(n, s)?).await
    }
}

/// Performs the data seeding operation using t
/// he specified distribution strategy.
async fn data_seeding(n: u64, strategy: impl DistributionStrategy) -> AppResult<()> {
    let mut generator = DataGenerator::new(strategy, SEED);
    let container = PostgresContainer::new();
    let id = create_and_start_container(container).await?;

    info!("Started PostgreSQL container with ID: {}", id);

    for _ in 0..n {
        info!("Generating zipfian value {:?}", generator.generate());
    }
    Ok(())
}
