use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Installs tracing subscriber for logging.
pub fn log_install() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
