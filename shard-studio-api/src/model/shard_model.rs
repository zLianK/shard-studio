use serde::Deserialize;

/// The strategy used to decide which shard each row is written to.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShardingAlgorithm {
    Hash,
}

/// Parameters for sharding.
#[derive(Debug, Deserialize)]
pub struct ShardRequest {
    /// Strategy used to assign rows to shards.
    pub strategy: ShardingAlgorithm,
    /// Number of shard instances.
    pub shards: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_is_deserialized() {
        let request: ShardRequest =
            serde_json::from_str(r#"{"strategy":"hash","shards":3}"#).unwrap();
        assert!(matches!(request.strategy, ShardingAlgorithm::Hash));
        assert_eq!(request.shards, 3);
    }

    #[test]
    fn unknown_strategy_is_rejected() {
        let request = serde_json::from_str::<ShardRequest>(r#"{"strategy":"range","shards":3}"#);
        assert!(request.is_err());
    }
}
