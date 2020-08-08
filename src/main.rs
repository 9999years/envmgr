use color_eyre::{
    eyre::{self, WrapErr},
    Section,
};
use tracing::{info, instrument};

mod data;
mod eval;

#[instrument]
fn main() -> eyre::Result<()> {
    install_tracing();
    color_eyre::install()?;

    println!("hi");

    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
