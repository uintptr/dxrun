use std::env;

use anyhow::{Result, bail};

use dxrun::command::run_command;
use log::LevelFilter;
use rstaples::logging::StaplesLogger;

fn main() -> Result<()> {
    StaplesLogger::new()
        .with_colors()
        .with_log_level(LevelFilter::Info)
        .start();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        bail!("Missing Command argument")
    }

    run_command(&args)
}
