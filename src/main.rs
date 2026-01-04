use anyhow::Result;

use clap::Parser;
use dxrun::{
    command::{UserArgs, run_command},
    config::display_config,
};
use log::LevelFilter;
use rstaples::logging::StaplesLogger;

#[tokio::main]
async fn main() -> Result<()> {
    let args = UserArgs::parse();

    StaplesLogger::new()
        .with_colors()
        .with_log_level(LevelFilter::Info)
        .start();

    match args.command {
        None => display_config(),
        Some(_) => run_command(&args).await,
    }
}
