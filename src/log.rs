use std::path::Path;

use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn log_init() {
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        std::fs::create_dir_all(log_dir).expect("Failed to create logs directory");
    }
    let file_appdender = rolling::daily(log_dir, "app.log");
    let (non_blocking_file, _guard) = non_blocking(file_appdender);
    let (non_blocking_stdout, _guard2) = non_blocking(std::io::stdout());
    let filter_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();
    let console_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(non_blocking_file)
        .with_filter(filter_filter);
    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(non_blocking_stdout)
        .compact()
        .with_filter(console_filter);
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
    std::mem::forget(_guard);
    std::mem::forget(_guard2);
}
