use std::fs::File;
use std::path::PathBuf;

use color_eyre::{
    eyre::{self, WrapErr},
    Section, SectionExt,
};
use structopt::StructOpt;
use tracing::{info, instrument};

mod data;
mod eval;
use eval::Eval;

#[instrument]
fn main() -> eyre::Result<()> {
    install_tracing();
    color_eyre::install()?;

    let opt = Opt::from_args();

    let cfg_file = &opt.config;

    let env_config: data::EnvConfig = serde_yaml::from_reader(
        File::open(cfg_file)
            .wrap_err("While opening config file")
            .with_section(|| format!("{:?}", cfg_file).header("Config file"))?,
    )
    .wrap_err("While deserializing config file")
    .with_section(|| format!("{:?}", cfg_file).header("Config file"))?;

    let vars = env_config
        .eval()
        .wrap_err("While evaluating environment variables")?;

    println!("{:#?}", vars);

    Ok(())
}

/// A helper for declaring common operations on environment variables (like "add
/// these directories to `$PATH` on these platforms / if they exist") in a
/// shell-agnostic way.
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,
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
