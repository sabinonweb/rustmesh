use crate::error::RustMeshError;

pub mod behaviour;
pub mod config;
pub mod error;
pub mod message;
pub mod transport;

pub type Result<T> = std::result::Result<T, RustMeshError>;

pub fn init_tracing(level: &str) {
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    // Checks for RUST_LOGS first, else fallbacks
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true).with_level(true))
        .init();
}
