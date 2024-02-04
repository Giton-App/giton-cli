use slog::o;
use slog::Drain;

use super::error::Result;

pub fn setup_logging() -> Result<slog_scope::GlobalLoggerGuard> {
    // Setup Logging
    let guard = slog_scope::set_global_logger(default_root_logger()?);
    slog_stdlog::init()?;

    Ok(guard)
}

pub fn default_root_logger() -> Result<slog::Logger> {
    // Create drains
    let drain = slog::Duplicate(default_discard()?, default_discard()?).fuse();

    // Create Logger
    let logger = slog::Logger::root(drain, o!("who" => "rust-starter"));

    // Return Logger
    Ok(logger)
}

fn default_discard() -> Result<slog_async::Async> {
    let drain = slog_async::Async::default(slog::Discard);

    Ok(drain)
}
