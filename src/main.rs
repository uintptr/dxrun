use anyhow::Result;

use clap::Parser;
use dxrun::{
    command::{UserArgs, run_command},
    config::display_config,
};
use log::LevelFilter;
use rstaples::logging::StaplesLogger;

fn main() -> Result<()> {
    let args = UserArgs::parse();

    StaplesLogger::new()
        .with_colors()
        .with_log_level(LevelFilter::Info)
        .start();

    if args.command.is_none() {
        display_config()
    } else {
        run_command(&args)
    }
}
