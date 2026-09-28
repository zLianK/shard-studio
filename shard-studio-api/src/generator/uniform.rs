use crate::{
    error::{AppError, AppResult},
    generator::strategy::DistributionStrategy,
};
use error_stack::ResultExt;
use rand::{RngExt, rngs::StdRng};
use rand_distr::Uniform;

/// A distribution strategy that generates values
/// according to a uniform distribution.
pub struct UniformStrategy {
    uniform: Uniform<u64>,
}

impl UniformStrategy {
    pub fn new(n: u64) -> AppResult<Self> {
        Ok(Self {
            uniform: Uniform::new_inclusive(1, n).change_context(
                AppError::SeedDistributionError("failed to create uniform strategy".to_string()),
            )?,
        })
    }
}

impl DistributionStrategy for UniformStrategy {
    fn next(&self, rng: &mut StdRng) -> u64 {
        rng.sample(self.uniform)
    }
}
