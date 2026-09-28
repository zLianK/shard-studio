use serde::Deserialize;

/// Parameters for uniform seeding.
#[derive(Debug, Deserialize)]
pub struct UniformSeed {
    pub n: u64,
}

/// Parameters for zipfian seeding.
#[derive(Debug, Deserialize)]
pub struct ZipfianSeed {
    pub n: u64,
    pub s: f64,
}
