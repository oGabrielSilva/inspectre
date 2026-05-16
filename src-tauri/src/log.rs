use tracing_subscriber::EnvFilter;

pub fn init() {
    let default_level = if cfg!(debug_assertions) { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}
