use crate::{
    error::{AppError, AppResult},
    sharding::strategy::ShardingStrategy,
};
use error_stack::bail;

/// FNV-1a 64-bit offset basis.
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// A sharding strategy that hashes the row key and
/// maps the hash to a shard with a modulo.
pub struct HashStrategy {
    shard_count: u64,
}

impl HashStrategy {
    /// Creates a strategy over `shard_count` shards.
    pub fn new(shard_count: u64) -> AppResult<Self> {
        if shard_count <= 1 {
            bail!(AppError::ShardingError(
                "the number of shards must be at least 2".into()
            ));
        }
        Ok(Self { shard_count })
    }
}

impl ShardingStrategy for HashStrategy {
    fn shard_for(&self, key: u64) -> usize {
        (fnv1a(key) % self.shard_count) as usize
    }
}

/// Hashes the bytes of the key with FNV-1a.
fn fnv1a(key: u64) -> u64 {
    key.to_le_bytes().iter().fold(FNV_OFFSET, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn less_than_two_shards_is_rejected() {
        assert!(HashStrategy::new(0).is_err());
        assert!(HashStrategy::new(1).is_err());
        assert!(HashStrategy::new(2).is_ok());
    }

    #[test]
    fn sequential_keys_are_spread_evenly() {
        let shards = 4;
        let keys = 10_000;
        let strategy = HashStrategy::new(shards).unwrap();

        let mut counts = vec![0u64; shards as usize];
        for key in 1..=keys {
            counts[strategy.shard_for(key)] += 1;
        }

        let expected = keys / shards;
        for count in counts {
            assert!(count.abs_diff(expected) <= expected / 10, "count {count}");
        }
    }

    #[test]
    fn shard_is_deterministic_and_in_range() {
        let strategy = HashStrategy::new(4).unwrap();
        for key in 1..=1000 {
            let shard = strategy.shard_for(key);
            assert!(shard < 4);
            assert_eq!(shard, strategy.shard_for(key));
        }
    }
}
