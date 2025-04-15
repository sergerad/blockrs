use clap::Parser;
use cli::{Cli, RpcType};
use color_eyre::Result;
use config::Config;
use providers::{eth::EthProvider, miden::MidenProvider};

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
    match args.rpc_type {
        RpcType::Eth => {
            let provider = EthProvider::new(args.rpc_url, &config.app.addresses)?;
            let mut app = App::new(args.tick_rate, args.frame_rate, provider, config)?;
            app.run().await.unwrap();
        }
        RpcType::Miden => {
            let provider = MidenProvider::new(args.rpc_url, &config.app.addresses)?;
            let mut app = App::new(args.tick_rate, args.frame_rate, provider, config)?;
            app.run().await.unwrap();
        }
    };
    Ok(())
}
