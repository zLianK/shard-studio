/// A trait that defines a sharding strategy.
pub trait ShardingStrategy: Send + Sync {
    /// Returns the index of the shard, in `0..shard_count`, that owns the given row key.
    fn shard_for(&self, key: u64) -> usize;
}
