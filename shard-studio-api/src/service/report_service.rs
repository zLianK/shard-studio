use crate::{
    error::{AppError, AppResult},
    model::report_model::{Report, SeedingReport},
};
use error_stack::{ResultExt, bail};
use std::path::PathBuf;
use tokio::{fs, sync::Mutex};
use tracing::info;

/// Directory where the experiment reports are written.
const REPORTS_DIR: &str = "reports";

/// Service responsible for keeping the report of the current experiment
/// and for persisting it.
#[derive(Debug)]
pub struct ReportService {
    current: Mutex<Report>,
}

impl ReportService {
    pub fn new() -> Self {
        Self {
            current: Mutex::new(Report::new()),
        }
    }

    /// Sets the seeding stage of the current report.
    pub async fn set_seeding(&self, seeding: SeedingReport) {
        self.current.lock().await.seeding = Some(seeding);
    }

    /// Writes the current report to disk and returns its name.
    pub async fn save_current(&self) -> AppResult<String> {
        let report = self.current.lock().await;
        if report.seeding.is_none() {
            bail!(AppError::NoCurrentReport);
        }

        save(&report).await?;
        info!("Saved report `{}`", report.name);
        Ok(report.name.clone())
    }

    /// Returns a copy of the current report.
    pub async fn current(&self) -> Report {
        self.current.lock().await.clone()
    }

    /// Discards the current report and starts the report of a new experiment.
    pub async fn reset(&self) {
        *self.current.lock().await = Report::new();
    }
}

/// Writes the report to `reports/<name>.json`.
async fn save(report: &Report) -> AppResult<()> {
    let path = report_path(&report.name);
    let json = serde_json::to_vec_pretty(report)
        .change_context(AppError::ReportError("failed to serialize report".into()))?;

    fs::create_dir_all(REPORTS_DIR)
        .await
        .change_context(AppError::ReportError(
            "failed to create reports directory".into(),
        ))?;
    fs::write(&path, json)
        .await
        .change_context(AppError::ReportError(format!(
            "failed to write report `{}`",
            report.name
        )))
}

/// Builds the path of the file of a report.
fn report_path(name: &str) -> PathBuf {
    PathBuf::from(REPORTS_DIR).join(format!("{name}.json"))
}
