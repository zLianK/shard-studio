use rand::rngs::StdRng;

/// A trait that defines a distribution strategy.
pub trait DistributionStrategy {
    /// Returns the next value in the distribution.
    fn next(&self, rng: &mut StdRng) -> u64;
}
