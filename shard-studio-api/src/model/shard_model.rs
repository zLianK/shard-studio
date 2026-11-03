use serde::{Deserialize, Serialize};

/// The strategy used to decide which shard each row is written to.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "lowercase"))]
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
    fn strategy_is_serialized_in_uppercase() {
        let json = serde_json::to_string(&ShardingAlgorithm::Hash).unwrap();
        assert_eq!(json, r#""HASH""#);
    }

    #[test]
    fn unknown_strategy_is_rejected() {
        let request = serde_json::from_str::<ShardRequest>(r#"{"strategy":"range","shards":3}"#);
        assert!(request.is_err());
    }
}
