use crate::model::{
    seed_model::{Distribution, SeedRequest},
    shard_model::{ShardRequest, ShardingAlgorithm},
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Prefix of the name of every experiment report.
const REPORT_NAME_PREFIX: &str = "shard-studio-experiment";

/// Report of a single experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub name: String,
    pub seeding: Option<SeedingReport>,
    pub sharding: Option<ShardingReport>,
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
            sharding: None,
        }
    }
}

/// Converts a duration to seconds.
fn duration_to_secs(duration: Duration) -> f64 {
    duration.as_millis() as f64 / 1000.0
}

/// Data collected in the seeding stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedingReport {
    pub distribution: Distribution,
    pub duration_secs: f64,
    pub key_histogram: Vec<KeyFrequency>,
}

impl SeedingReport {
    /// Creates the report of the seeding stage.
    pub fn new(request: &SeedRequest) -> Self {
        Self {
            distribution: request.distribution,
            duration_secs: 0.0,
            key_histogram: Vec::new(),
        }
    }

    /// Sets the number of rows stored under each key.
    pub fn key_histogram(mut self, key_histogram: Vec<KeyFrequency>) -> Self {
        self.key_histogram = key_histogram;
        self
    }

    /// Sets the time taken to insert the rows.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration_secs = duration_to_secs(duration);
        self
    }
}

/// Data collected in the sharding stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardingReport {
    pub strategy: ShardingAlgorithm,
    pub shards: u64,
    pub duration_secs: f64,
}

impl ShardingReport {
    /// Creates the report of the sharding stage.
    pub fn new(request: &ShardRequest) -> Self {
        Self {
            strategy: request.strategy,
            shards: request.shards,
            duration_secs: 0.0,
        }
    }

    /// Sets the time taken to distribute the rows.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration_secs = duration_to_secs(duration);
        self
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
                duration_secs: 1.234,
                key_histogram: vec![KeyFrequency { key: 1, count: 3 }],
            }),
            sharding: Some(ShardingReport {
                strategy: ShardingAlgorithm::Hash,
                shards: 4,
                duration_secs: 0.5,
            }),
        };

        assert_eq!(
            serde_json::to_value(&report).unwrap(),
            json!({
                "name": "experiment",
                "seeding": {
                    "distribution": "UNIFORM",
                    "duration_secs": 1.234,
                    "key_histogram": [{ "key": 1, "count": 3 }],
                },
                "sharding": { "strategy": "HASH", "shards": 4, "duration_secs": 0.5 },
            })
        );
    }

    #[test]
    fn stages_not_run_yet_are_serialized_as_null() {
        let value = serde_json::to_value(Report::new()).unwrap();
        assert!(value["seeding"].is_null());
        assert!(value["sharding"].is_null());
    }

    #[test]
    fn sharding_report_keeps_the_parameters_of_the_request() {
        let request = ShardRequest {
            strategy: ShardingAlgorithm::Hash,
            shards: 3,
        };

        let report = ShardingReport::new(&request).duration(Duration::from_millis(1500));

        assert!(matches!(report.strategy, ShardingAlgorithm::Hash));
        assert_eq!(report.shards, 3);
        assert_eq!(report.duration_secs, 1.5);
    }

    #[test]
    fn duration_is_reported_in_seconds_with_milliseconds() {
        assert_eq!(duration_to_secs(Duration::from_micros(1_234_999)), 1.234);
        assert_eq!(duration_to_secs(Duration::from_millis(7)), 0.007);
        assert_eq!(duration_to_secs(Duration::ZERO), 0.0);
    }
}
