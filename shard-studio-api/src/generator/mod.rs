use crate::generator::strategy::DistributionStrategy;
use rand::{SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

pub mod hotspot;
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

impl DataObject {
    #[cfg(test)]
    pub fn new(id: u64, text: &str) -> Self {
        let text = text.to_string();
        Self { id, text }
    }

    /// Returns the bytes of the data object as a line of
    /// the database COPY text format.
    pub fn into_row(&self) -> Vec<u8> {
        format!("{}\t{}\n", self.id, self.text).into_bytes()
    }
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
        Self {
            strategy,
            rng,
            text,
        }
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
    use crate::generator::hotspot::HotspotStrategy;
    use crate::generator::uniform::UniformStrategy;
    use crate::generator::zipfian::ZipfianStrategy;

    #[test]
    fn row_is_a_tab_separated_line() {
        let object = DataObject::new(7, "abc");
        assert_eq!(object.into_row(), b"7\tabc\n");
    }

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

    #[test]
    fn generate_hotspot_test() {
        let number_of_elements = 10_000;
        let k = 1000;

        let strategy = HotspotStrategy::new(k).unwrap();
        let mut generator = DataGenerator::new(strategy, 42);

        let mut hot = 0;
        for _ in 0..number_of_elements {
            let data = generator.generate();
            assert_eq!(data.text.len(), TEXT_SIZE);
            assert!(data.id >= 1 && data.id <= k);
            if data.id <= k / 5 {
                hot += 1;
            }
        }
        let ratio = hot as f64 / number_of_elements as f64;
        assert!((0.78..=0.82).contains(&ratio), "hot ratio was {ratio}");
    }

    #[test]
    fn same_seed_generates_the_same_keys() {
        let keys = |seed| {
            let strategy = ZipfianStrategy::new(100).unwrap();
            let mut generator = DataGenerator::new(strategy, seed);
            (0..50).map(|_| generator.generate().id).collect::<Vec<_>>()
        };

        assert_eq!(keys(42), keys(42));
        assert_ne!(keys(42), keys(43));
    }

    #[test]
    fn zipfian_concentrates_rows_on_the_first_keys() {
        let strategy = ZipfianStrategy::new(1000).unwrap();
        let mut generator = DataGenerator::new(strategy, 42);

        let mut first = 0;
        let mut last = 0;
        for _ in 0..10_000 {
            match generator.generate().id {
                1 => first += 1,
                1000 => last += 1,
                _ => {}
            }
        }
        assert!(first > last * 10);
    }

    #[test]
    fn strategies_reject_an_empty_key_space() {
        assert!(UniformStrategy::new(0).is_err());
        assert!(ZipfianStrategy::new(0).is_err());
        assert!(HotspotStrategy::new(0).is_err());
    }
}
