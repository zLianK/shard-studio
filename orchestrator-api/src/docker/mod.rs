pub mod postgres;

use crate::error::{AppError, AppResult};
use bollard::{
    Docker,
    plugin::ContainerCreateBody,
    query_parameters::{CreateContainerOptionsBuilder, CreateImageOptionsBuilder},
};
use error_stack::ResultExt;
use futures_util::StreamExt;

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
