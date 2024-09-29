use tracing::Level;
use tracing_subscriber::{fmt as TracingFmt, fmt::time::UtcTime};

pub struct LoggingConfig;

impl LoggingConfig {
    pub fn init(&self) {
        TracingFmt()
            .json()
            .with_max_level(Level::DEBUG)
            .with_timer(UtcTime::rfc_3339())
            .with_target(false)
            .init();
    }
}
