use std::error::Error;
mod cli;
mod config;
mod core;
mod models;
mod mutation;
mod query;
mod service;
mod utils;

use clap::Parser;
use cli::Cli;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();
    let config = Config::from_env()?;
    core::run(cli, &config).await?;

    Ok(())
}
