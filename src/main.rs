use std::{env, fs};

use anyhow::Result;

use dxrun::{command::run_command, config::get_config_dir};
use log::LevelFilter;
use rstaples::logging::StaplesLogger;

fn list_config() -> Result<Vec<String>> {
    let mut filenames = vec![];

    let config_dir = get_config_dir()?;

    for entry in fs::read_dir(config_dir)? {
        if let Ok(entry) = entry {
            if let Ok(filename) = entry.file_name().into_string() {
                filenames.push(filename)
            }
        }
    }

    Ok(filenames)
}

fn display_config() -> Result<()> {
    println!("Available Options:");
    for config in list_config()? {
        println!("- {config}");
    }

    Ok(())
}

fn main() -> Result<()> {
    StaplesLogger::new()
        .with_colors()
        .with_log_level(LevelFilter::Info)
        .start();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        display_config()
    } else {
        run_command(&args)
    }
}
