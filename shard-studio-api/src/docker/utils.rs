/// Prefix used for all container names created by Shard Studio.
const CONTAINER_NAME_PREFIX: &str = "shard-studio";

/// Returns the name for the main container.
pub fn container_name_main() -> String {
    format!("{CONTAINER_NAME_PREFIX}-main")
}

/// Returns the name for the container of the shard with the given index.
pub fn container_name_shard(index: usize) -> String {
    format!("{CONTAINER_NAME_PREFIX}-shard-{index}")
}

/// Returns the prefix shared by all container names created by Shard Studio.
pub fn container_name_prefix() -> String {
    CONTAINER_NAME_PREFIX.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_names_start_with_the_shared_prefix() {
        let prefix = container_name_prefix();
        assert!(container_name_main().starts_with(&prefix));
        assert!(container_name_shard(0).starts_with(&prefix));
    }

    #[test]
    fn container_names_are_distinct() {
        assert_eq!(container_name_shard(3), "shard-studio-shard-3");
        assert_ne!(container_name_shard(0), container_name_shard(1));
        assert_ne!(container_name_main(), container_name_shard(0));
    }
}
