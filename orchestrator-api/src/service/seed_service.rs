use crate::{
    docker::{create_and_start_container, postgres::PostgresContainer, utils::container_name_main},
    error::AppResult,
    generator::{
        DataGenerator, DataObject, strategy::DistributionStrategy, uniform::UniformStrategy,
        zipfian::ZipfianStrategy,
    },
    repository::postgres::PostgresRepository,
};
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Number of generated items buffered before being written to the database.
const BATCH_SIZE: usize = 1000;

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
    let container = PostgresContainer::new(container_name_main());
    let host_port = container.host_port();
    let id = create_and_start_container(container).await?;

    info!("Started PostgreSQL container with ID: {}", id);

    let repository = PostgresRepository::connect(host_port).await?;
    repository.create_table().await?;

    let mut buffer: Vec<DataObject> = Vec::with_capacity(BATCH_SIZE);
    for _ in 0..n {
        buffer.push(generator.generate());
        if buffer.len() == BATCH_SIZE {
            repository.save_all(&buffer).await?;
            buffer.clear();
        }
    }
    repository.save_all(&buffer).await?;

    info!("Seeded {} data objects into PostgreSQL", n);
    Ok(())
}
