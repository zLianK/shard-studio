/// Prefix used for all container names created by Shard Studio.
const CONTAINER_NAME_PREFIX: &str = "shard-studio";

/// Returns the name for the main container.
pub fn container_name_main() -> String {
    format!("{CONTAINER_NAME_PREFIX}-main")
}

/// Returns the prefix shared by all container names created by Shard Studio.
pub fn container_name_prefix() -> String {
    CONTAINER_NAME_PREFIX.into()
}
