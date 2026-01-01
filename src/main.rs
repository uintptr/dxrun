use anyhow::{Result, bail};

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

    if args.extras.is_empty() {
        display_config()
    } else if args.extras.len() != 1 {
        bail!("invalid # of arguments");
    } else {
        run_command(&args)
    }
}
