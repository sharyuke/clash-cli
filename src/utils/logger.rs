use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();
}
