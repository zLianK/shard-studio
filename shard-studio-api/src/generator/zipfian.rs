use crate::{
    error::{AppError, AppResult},
    generator::strategy::DistributionStrategy,
};
use error_stack::ResultExt;
use rand::{RngExt, rngs::StdRng};
use rand_distr::Zipf;

/// The Zipfian skew constant (`s`), the same default used by YCSB.
pub const ZIPFIAN_CONSTANT: f64 = 0.99;

/// A distribution strategy that generates values
/// according to a Zipfian distribution.
pub struct ZipfianStrategy {
    zipf: Zipf<f64>,
}

impl ZipfianStrategy {
    /// Creates a strategy over a key space of `k` keys.
    pub fn new(k: u64) -> AppResult<Self> {
        Ok(Self {
            zipf: Zipf::new(k as f64, ZIPFIAN_CONSTANT).change_context(AppError::SeedDistributionError(
                "failed to create zipfian strategy".to_string(),
            ))?,
        })
    }
}

impl DistributionStrategy for ZipfianStrategy {
    fn next(&self, rng: &mut StdRng) -> u64 {
        rng.sample(self.zipf) as u64
    }
}
