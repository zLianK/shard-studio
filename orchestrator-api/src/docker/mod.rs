pub mod postgres;
pub mod utils;

use crate::error::{AppError, AppResult};
use bollard::{
    Docker,
    errors::Error as DockerClientError,
    models::ContainerSummary,
    plugin::ContainerCreateBody,
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, ListContainersOptionsBuilder,
        RemoveContainerOptionsBuilder,
    },
};
use error_stack::ResultExt;
use futures_util::StreamExt;
use std::collections::HashMap;

/// Trait that defines the necessary methods for
/// creating and managing Docker containers.
pub trait DockerContainer {
    /// Returns the Docker image for the container.
    fn image(&self) -> String;
    /// Returns the name of the container.
    fn name(&self) -> String;
    /// Returns the body for creating the container.
    fn body(&self) -> ContainerCreateBody;
}

fn get_docker_daemon() -> AppResult<Docker> {
    Docker::connect_with_local_defaults().change_context(AppError::DockerError(
        "failed to connect to docker daemon".into(),
    ))
}

/// Creates and starts a new container.
pub async fn create_and_start_container(container: impl DockerContainer) -> AppResult<String> {
    let docker = get_docker_daemon()?;

    let image = container.image();

    let options = CreateImageOptionsBuilder::new().from_image(&image).build();
    let mut stream = docker.create_image(Some(options), None, None);
    while let Some(result) = stream.next().await {
        result.change_context(AppError::DockerError(format!(
            "failed to pull image '{image}'"
        )))?;
    }

    let name = container.name();

    let options = CreateContainerOptionsBuilder::new().name(&name).build();
    let response = docker
        .create_container(Some(options), container.body())
        .await
        .change_context(AppError::DockerError(format!(
            "failed to create container '{name}'",
        )))?;

    docker
        .start_container(&response.id, None)
        .await
        .change_context(AppError::DockerError(format!(
            "failed to start container '{name}'"
        )))?;

    Ok(response.id)
}

/// Removes all containers whose name starts with the given prefix.
pub async fn remove_containers_with_prefix(prefix: &str) -> AppResult<()> {
    let containers = list_containers_with_prefix(prefix).await?;

    for container in containers {
        if let Some(id) = container.id {
            remove_container(&id).await?;
        }
    }

    Ok(())
}

/// Lists containers whose name starts with the given prefix.
async fn list_containers_with_prefix(prefix: &str) -> AppResult<Vec<ContainerSummary>> {
    let docker = get_docker_daemon()?;

    let filters = HashMap::from([("name".to_string(), vec![prefix.to_string()])]);
    let options = ListContainersOptionsBuilder::new()
        .all(true)
        .filters(&filters)
        .build();

    docker
        .list_containers(Some(options))
        .await
        .change_context(AppError::DockerError(format!(
            "failed to list containers with prefix `{prefix}`"
        )))
}

/// Removes a container by name.
async fn remove_container(name: &str) -> AppResult<()> {
    let docker = get_docker_daemon()?;

    let options = RemoveContainerOptionsBuilder::new().force(true).build();

    match docker.remove_container(name, Some(options)).await {
        Ok(()) => Ok(()),
        Err(DockerClientError::DockerResponseServerError {
            status_code: 404, ..
        }) => Ok(()),
        Err(err) => Err(err).change_context(AppError::DockerError(format!(
            "failed to remove container `{name}`"
        ))),
    }
}
