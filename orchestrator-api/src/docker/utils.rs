/// Prefix used for all container names created by the simulator.
const CONTAINER_NAME_PREFIX: &str = "sharding-simulator";

/// Returns the name for the main container.
pub fn container_name_main() -> String {
    format!("{CONTAINER_NAME_PREFIX}-main")
}

/// Returns the prefix shared by all container names created by the simulator.
pub fn container_name_prefix() -> String {
    CONTAINER_NAME_PREFIX.into()
}
