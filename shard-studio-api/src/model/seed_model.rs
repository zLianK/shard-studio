use serde::Deserialize;

/// The distribution used to generate the keys of the seeded rows.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    Uniform,
    Zipfian,
    Hotspot,
}

/// Parameters for seeding.
#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// Distribution of the keys.
    pub distribution: Distribution,
    /// Number of rows to insert.
    pub n: u64,
    /// Size of the key space.
    pub k: u64,
}
