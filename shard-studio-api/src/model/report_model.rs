use crate::{
    error::AppResult,
    model::seed_model::{Distribution, SeedRequest},
    repository::postgres::PostgresRepository,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Prefix of the name of every experiment report.
const REPORT_NAME_PREFIX: &str = "shard-studio-experiment";

/// Report of a single experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub name: String,
    pub seeding: Option<SeedingReport>,
}

impl Report {
    /// Creates the empty report of a new experiment, named after the current timestamp.
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let name = format!("{REPORT_NAME_PREFIX}-{timestamp}");
        Self {
            name,
            seeding: None,
        }
    }
}

/// Data collected in the seeding stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedingReport {
    pub distribution: Distribution,
    pub key_histogram: Vec<KeyFrequency>,
}

impl SeedingReport {
    /// Creates the report of the seeding stage
    pub async fn new(request: &SeedRequest, repository: &PostgresRepository) -> AppResult<Self> {
        Ok(Self {
            distribution: request.distribution,
            key_histogram: repository.key_histogram().await?,
        })
    }
}

/// Number of rows stored under a single key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyFrequency {
    pub key: i64,
    pub count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new_report_is_named_after_the_prefix_and_has_no_seeding() {
        let report = Report::new();
        assert!(report.name.starts_with(REPORT_NAME_PREFIX));
        assert!(report.seeding.is_none());
    }

    #[test]
    fn report_is_serialized_with_the_shape_read_by_the_dashboards() {
        let report = Report {
            name: "experiment".into(),
            seeding: Some(SeedingReport {
                distribution: Distribution::Uniform,
                key_histogram: vec![KeyFrequency { key: 1, count: 3 }],
            }),
        };

        assert_eq!(
            serde_json::to_value(&report).unwrap(),
            json!({
                "name": "experiment",
                "seeding": {
                    "distribution": "uniform",
                    "key_histogram": [{ "key": 1, "count": 3 }],
                },
            })
        );
    }
}
