use serde::{Deserialize, Serialize};

/// The distribution used to generate the keys of the seeded rows.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_is_deserialized() {
        let json = r#"{"distribution":"zipfian","n":1000,"k":50}"#;
        let request: SeedRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(request.distribution, Distribution::Zipfian));
        assert_eq!(request.n, 1000);
        assert_eq!(request.k, 50);
    }

    #[test]
    fn invalid_requests_are_rejected() {
        let invalid = [
            r#"{"distribution":"normal","n":1,"k":1}"#,
            r#"{"distribution":"uniform","n":-1,"k":1}"#,
            r#"{"distribution":"uniform","n":1}"#,
        ];
        for json in invalid {
            assert!(serde_json::from_str::<SeedRequest>(json).is_err(), "{json}");
        }
    }

    #[test]
    fn distribution_is_serialized_in_lowercase() {
        let json = serde_json::to_string(&Distribution::Hotspot).unwrap();
        assert_eq!(json, r#""hotspot""#);
    }
}
