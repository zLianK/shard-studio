use crate::generator::strategy::DistributionStrategy;
use rand::{SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

pub mod strategy;
pub mod uniform;
pub mod zipfian;

/// The size, in bytes, of the text of every data object.
pub const TEXT_SIZE: usize = 256;

/// A data object that represents a single unit of data in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataObject {
    pub id: u64,
    pub text: String,
}

/// A data generator that uses a distribution strategy
/// to generate data objects.
pub struct DataGenerator<T: DistributionStrategy> {
    strategy: T,
    rng: StdRng,
    text: String,
}

impl<T: DistributionStrategy> DataGenerator<T> {
    pub fn new(strategy: T, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let text = "a".repeat(TEXT_SIZE);
        Self { strategy, rng, text }
    }

    /// Generates a data object using the distribution strategy.
    /// The text is always the same, [`TEXT_SIZE`] bytes long.
    pub fn generate(&mut self) -> DataObject {
        let id = self.strategy.next(&mut self.rng);
        DataObject {
            id,
            text: self.text.clone(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::generator::uniform::UniformStrategy;
    use crate::generator::zipfian::ZipfianStrategy;

    #[test]
    fn generate_zipfian_test() {
        let number_of_elements = 1000;

        let strategy = ZipfianStrategy::new(number_of_elements).unwrap();
        let mut generator = DataGenerator::new(strategy, 42);

        for _ in 0..number_of_elements {
            let data = generator.generate();
            assert_eq!(data.text.len(), TEXT_SIZE);
            assert!(data.id >= 1 && data.id <= number_of_elements);
        }
    }

    #[test]
    fn generate_uniform_test() {
        let number_of_elements = 1000;

        let strategy = UniformStrategy::new(number_of_elements).unwrap();
        let mut generator = DataGenerator::new(strategy, 42);

        for _ in 0..number_of_elements {
            let data = generator.generate();
            assert_eq!(data.text.len(), TEXT_SIZE);
            assert!(data.id >= 1 && data.id <= number_of_elements);
        }
    }
}
