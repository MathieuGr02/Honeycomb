use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger(level: LevelFilter) {
    let stdout_log = fmt::layer()
        .with_thread_names(true)
        .with_line_number(true)
        .without_time();

    tracing_subscriber::registry()
        .with(level)
        .with(stdout_log)
        .init();
}
