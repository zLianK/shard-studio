use crate::{
    docker::{create_and_start_container, postgres::PostgresContainer, utils::container_name_main},
    error::AppResult,
    generator::{
        DataGenerator, DataObject, hotspot::HotspotStrategy, strategy::DistributionStrategy,
        uniform::UniformStrategy, zipfian::ZipfianStrategy,
    },
    model::{
        report_model::SeedingReport,
        seed_model::{Distribution, SeedRequest},
    },
    repository::postgres::PostgresRepository,
    service::report_service::ReportService,
};
use std::sync::Arc;
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Number of generated items buffered before being written to the database.
const BATCH_SIZE: usize = 1000;

/// Service responsible for handling seeding operations.
#[derive(Debug, Clone)]
pub struct SeedService {
    report_service: Arc<ReportService>,
}

impl SeedService {
    pub fn new(report_service: Arc<ReportService>) -> Self {
        Self { report_service }
    }

    /// Handles the seeding request: `n` rows with ids in `1..=k`
    /// following the requested distribution.
    pub async fn seed(&self, request: SeedRequest) -> AppResult<()> {
        let repository = start_repository().await?;
        data_seeding(&repository, &request).await?;
        let seeding = SeedingReport::new(&request, &repository).await?;
        self.report_service.set_seeding(seeding).await;
        Ok(())
    }
}

/// Starts the main PostgreSQL container and returns a repository
/// connected to it, with the table already created.
async fn start_repository() -> AppResult<PostgresRepository> {
    let container = PostgresContainer::new(container_name_main());
    let host_port = container.host_port();
    let id = create_and_start_container(container).await?;

    info!("Started PostgreSQL container with ID: {}", id);

    let repository = PostgresRepository::connect(host_port).await?;
    repository.create_table().await?;

    Ok(repository)
}

/// Inserts the rows described by the request into the repository.
async fn data_seeding(repository: &PostgresRepository, request: &SeedRequest) -> AppResult<()> {
    let SeedRequest { distribution, n, k } = *request;

    let strategy: Box<dyn DistributionStrategy> = match distribution {
        Distribution::Uniform => Box::new(UniformStrategy::new(k)?),
        Distribution::Zipfian => Box::new(ZipfianStrategy::new(k)?),
        Distribution::Hotspot => Box::new(HotspotStrategy::new(k)?),
    };

    let mut generator = DataGenerator::new(strategy, SEED);
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
