use tracing::Level;
use tracing_subscriber::{fmt as TracingFmt, fmt::time::UtcTime};

pub struct LoggingConfig;

impl LoggingConfig {
    // Initialize the logging system
    //
    // # Arguments:
    // - self.
    // - level: Log level as a &str.
    //
    // # Returns:
    // - No return value.
    pub fn init(&self) {
        TracingFmt()
            .json()
            .with_max_level(Level::DEBUG)
            .with_timer(UtcTime::rfc_3339())
            .with_target(false)
            .init();
    }
}
