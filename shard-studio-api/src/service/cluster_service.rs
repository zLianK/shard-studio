use crate::{
    docker::{
        create_and_start_container,
        postgres::PostgresContainer,
        remove_containers_with_prefix,
        utils::{container_name_main, container_name_prefix, container_name_shard},
    },
    error::{AppError, AppResult},
    repository::postgres::PostgresRepository,
};
use error_stack::bail;
use tokio::sync::Mutex;
use tracing::info;

/// Databases of the current simulation.
#[derive(Debug, Default)]
struct Cluster {
    main: Option<PostgresRepository>,
    shards: Vec<PostgresRepository>,
}

/// Service responsible for starting the databases
/// of the simulation and for keeping them until
/// the simulation is reset.
#[derive(Debug)]
pub struct ClusterService {
    cluster: Mutex<Cluster>,
}

impl ClusterService {
    pub fn new() -> Self {
        Self {
            cluster: Mutex::new(Cluster::default()),
        }
    }

    /// Starts the main database and keeps it in the cluster.
    pub async fn start_main(&self) -> AppResult<PostgresRepository> {
        let mut cluster = self.cluster.lock().await;
        if cluster.main.is_some() {
            bail!(AppError::ClusterError(
                "the main database is already running".into()
            ));
        }

        let container_name = container_name_main();
        let repository = start_repository(container_name).await?;
        cluster.main = Some(repository.clone());
        Ok(repository)
    }

    /// Returns the main database.
    pub async fn main(&self) -> AppResult<PostgresRepository> {
        match &self.cluster.lock().await.main {
            Some(repository) => Ok(repository.clone()),
            None => bail!(AppError::NoMainDatabase),
        }
    }

    /// Starts `count` shard databases and keeps them in the cluster.
    /// The position of each repository is the index of its shard.
    pub async fn start_shards(&self, count: usize) -> AppResult<Vec<PostgresRepository>> {
        let mut cluster = self.cluster.lock().await;
        if cluster.main.is_none() {
            bail!(AppError::NoMainDatabase);
        }
        if !cluster.shards.is_empty() {
            bail!(AppError::ClusterError(
                "the shards are already running".into()
            ));
        }

        for index in 0..count {
            let container_name = container_name_shard(index);
            let repository = start_repository(container_name).await?;
            cluster.shards.push(repository);
        }
        Ok(cluster.shards.clone())
    }

    /// Closes the connections to every database, forgets them and
    /// removes all containers created by the application.
    pub async fn reset(&self) -> AppResult<()> {
        let mut cluster = self.cluster.lock().await;
        let main = cluster.main.take();
        let shards = std::mem::take(&mut cluster.shards);
        for repository in main.iter().chain(shards.iter()) {
            repository.close().await;
        }
        let prefix = container_name_prefix();
        remove_containers_with_prefix(&prefix).await
    }
}

/// Starts a PostgreSQL container with the given name and returns a
/// repository connected to it, with the table already created.
async fn start_repository(name: String) -> AppResult<PostgresRepository> {
    let container = PostgresContainer::new(name);
    let host_port = container.host_port();
    let id = create_and_start_container(container).await?;

    info!("Started PostgreSQL container with ID: {}", id);

    let repository = PostgresRepository::connect(host_port).await?;
    repository.create_table().await?;

    Ok(repository)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main_is_missing_before_the_seeding() {
        let service = ClusterService::new();
        let error = service.main().await.unwrap_err();
        assert!(matches!(error.current_context(), AppError::NoMainDatabase));
    }

    #[tokio::test]
    async fn shards_cannot_start_without_the_main_database() {
        let service = ClusterService::new();
        let error = service.start_shards(2).await.unwrap_err();
        assert!(matches!(error.current_context(), AppError::NoMainDatabase));
    }
}
