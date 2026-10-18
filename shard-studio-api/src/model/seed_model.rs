use serde::Deserialize;

/// Parameters for uniform seeding.
#[derive(Debug, Deserialize)]
pub struct UniformSeed {
    /// Number of rows to insert.
    pub n: u64,
    /// Size of the key space.
    pub k: u64,
}

/// Parameters for hotspot seeding.
#[derive(Debug, Deserialize)]
pub struct HotspotSeed {
    /// Number of rows to insert.
    pub n: u64,
    /// Size of the key space.
    pub k: u64,
}

/// Parameters for zipfian seeding.
#[derive(Debug, Deserialize)]
pub struct ZipfianSeed {
    /// Number of rows to insert.
    pub n: u64,
    /// Size of the key space.
    pub k: u64,
}
