use crate::generator::strategy::DistributionStrategy;
use fake::{Fake, faker::lorem::en::Sentence};
use rand::{SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

pub mod strategy;
pub mod uniform;
pub mod zipfian;

/// The number of words in the generated text.
const NUMBER_OF_WORDS: usize = 25;

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
}

impl<T: DistributionStrategy> DataGenerator<T> {
    pub fn new(strategy: T, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        Self { strategy, rng }
    }

    /// Generates a data object using the distribution strategy.
    pub fn generate(&mut self) -> DataObject {
        let id = self.strategy.next(&mut self.rng);
        let range = NUMBER_OF_WORDS..(NUMBER_OF_WORDS + 1);
        let text: String = Sentence(range).fake_with_rng(&mut self.rng);
        DataObject { id, text }
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

        let strategy = ZipfianStrategy::new(number_of_elements, 1.0).unwrap();
        let mut generator = DataGenerator::new(strategy, 42);

        for _ in 0..number_of_elements {
            let data = generator.generate();
            assert!(data.text.len() > 128 && data.text.len() < 256);
            assert!(data.id >= 1 && data.id <= number_of_elements);
            assert!(data.text.split(" ").count() == NUMBER_OF_WORDS);
        }
    }

    #[test]
    fn generate_uniform_test() {
        let number_of_elements = 1000;

        let strategy = UniformStrategy::new(number_of_elements).unwrap();
        let mut generator = DataGenerator::new(strategy, 42);

        for _ in 0..number_of_elements {
            let data = generator.generate();
            assert!(data.text.len() > 128 && data.text.len() < 256);
            assert!(data.id >= 1 && data.id <= number_of_elements);
            assert!(data.text.split(" ").count() == NUMBER_OF_WORDS);
        }
    }
}
