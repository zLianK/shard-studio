use crate::{
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
    service::{cluster_service::ClusterService, report_service::ReportService},
};
use std::{sync::Arc, time::Instant};
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Number of generated items buffered before being written to the database.
const BATCH_SIZE: usize = 1000;

/// Service responsible for handling seeding operations.
#[derive(Debug, Clone)]
pub struct SeedService {
    cluster_service: Arc<ClusterService>,
    report_service: Arc<ReportService>,
}

impl SeedService {
    pub fn new(cluster_service: Arc<ClusterService>, report_service: Arc<ReportService>) -> Self {
        Self {
            cluster_service,
            report_service,
        }
    }

    /// Handles the seeding request: `n` rows with ids in `1..=k`
    /// following the requested distribution.
    pub async fn seed(&self, request: SeedRequest) -> AppResult<()> {
        let repository = self.cluster_service.start_main().await?;

        let started = Instant::now();
        data_seeding(&repository, &request).await?;
        let elapsed = started.elapsed();

        let seeding = SeedingReport::new(&request)
            .key_histogram(repository.key_histogram().await?)
            .duration(elapsed);
        self.report_service.set_seeding(seeding).await;

        Ok(())
    }
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
