use crate::docker::DockerContainer;
use bollard::plugin::{ContainerCreateBody, HostConfig, PortBinding};
use std::collections::HashMap;

/// The PostgreSQL image used for creating containers.
const POSTGRES_IMAGE: &str = "postgres:16-alpine";
/// The port on which PostgreSQL is exposed within the container.
const POSTGRES_PORT: u16 = 5433;

/// PostgreSQL docker container information and configuration.
pub struct PostgresContainer {
    image: String,
    name: String,
    body: ContainerCreateBody,
}

impl PostgresContainer {
    pub fn new() -> Self {
        let image = POSTGRES_IMAGE.into();
        let port = next_postgres_port();
        let name = format!("postgres-{port}");
        let body = postgres_create_container_body(port);
        Self { image, name, body }
    }
}

impl DockerContainer for PostgresContainer {
    fn image(&self) -> String {
        self.image.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn body(&self) -> ContainerCreateBody {
        self.body.clone()
    }
}

/// Creates the container body for a PostgreSQL container.
fn postgres_create_container_body(port: u16) -> ContainerCreateBody {
    let container_port = format!("{POSTGRES_PORT}/tcp");

    let port_bindings = HashMap::from([(
        container_port.clone(),
        Some(vec![PortBinding {
            host_ip: None,
            host_port: Some(port.to_string()),
        }]),
    )]);

    ContainerCreateBody {
        image: Some(POSTGRES_IMAGE.into()),
        env: Some(vec![
            "POSTGRES_USER=username".into(),
            "POSTGRES_PASSWORD=password".into(),
            "POSTGRES_DB=postgres".into(),
        ]),
        exposed_ports: Some(vec![container_port]),
        host_config: Some(HostConfig {
            port_bindings: Some(port_bindings),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Returns the next available port number starting from 5432.
fn next_postgres_port() -> u16 {
    use std::sync::atomic::{AtomicU16, Ordering};
    static PORT: AtomicU16 = AtomicU16::new(POSTGRES_PORT);
    PORT.fetch_add(1, Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_postgres_port_test() {
        let port = next_postgres_port();
        assert_eq!(port, POSTGRES_PORT);
        let port = next_postgres_port();
        assert_eq!(port, POSTGRES_PORT + 1);
    }
}
