use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use config::Config;
use providers::eth::EthProvider;

use crate::app::App;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod monitor;
mod providers;
mod tui;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let config = Config::new()?;
    let provider = EthProvider::new(args.rpc_url, &config.app.addresses)?;
    let mut app = App::new(args.tick_rate, args.frame_rate, provider, config)?;
    app.run().await?;
    Ok(())
}
