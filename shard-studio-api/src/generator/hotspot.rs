use crate::{
    error::{AppError, AppResult},
    generator::strategy::DistributionStrategy,
};
use error_stack::ResultExt;
use rand::{RngExt, rngs::StdRng};
use rand_distr::Uniform;

/// Fraction of the key space that forms the hot interval.
pub const HOTSPOT_KEY_FRACTION: f64 = 0.2;

/// Fraction of the rows that land inside the hot interval.
pub const HOTSPOT_ROW_FRACTION: f64 = 0.8;

/// A distribution strategy where a fixed interval of 20% of keys
/// concentrates 80% of the rows. Keys inside each
/// interval are chosen uniformly.
pub struct HotspotStrategy {
    hot: Uniform<u64>,
    cold: Uniform<u64>,
}

impl HotspotStrategy {
    /// Creates a strategy over a key space of `k` keys.
    pub fn new(k: u64) -> AppResult<Self> {
        let hot_size = (k as f64 * HOTSPOT_KEY_FRACTION).ceil() as u64;

        let hot = Uniform::new_inclusive(1, hot_size).change_context(
            AppError::SeedDistributionError("failed to create hotspot hot interval".to_string()),
        )?;

        let cold = Uniform::new_inclusive(hot_size + 1, k).change_context(
            AppError::SeedDistributionError("failed to create hotspot cold interval".to_string()),
        )?;

        Ok(Self { hot, cold })
    }
}

impl DistributionStrategy for HotspotStrategy {
    fn next(&self, rng: &mut StdRng) -> u64 {
        if rng.random_bool(HOTSPOT_ROW_FRACTION) {
            rng.sample(self.hot)
        } else {
            rng.sample(self.cold)
        }
    }
}
