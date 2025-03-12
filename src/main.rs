use clap::Parser;
use cli::Cli;
use color_eyre::Result;
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
    let provider = EthProvider::new(args.rpc_url);
    let mut app = App::new(args.tick_rate, args.frame_rate, provider)?;
    app.run().await?;
    Ok(())
}
