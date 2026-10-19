use rand::rngs::StdRng;

/// A trait that defines a distribution strategy.
pub trait DistributionStrategy: Send {
    /// Returns the next value in the distribution.
    fn next(&self, rng: &mut StdRng) -> u64;
}

impl DistributionStrategy for Box<dyn DistributionStrategy> {
    fn next(&self, rng: &mut StdRng) -> u64 {
        (**self).next(rng)
    }
}
