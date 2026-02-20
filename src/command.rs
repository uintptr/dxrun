use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, anyhow, bail};
use clap::Parser;
use log::info;
use which::which;

use crate::config::get_config_dir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct UserArgs {
    /// Extra Volume Configuration
    #[arg(short, long, action = clap::ArgAction::Append)]
    pub volume: Option<Vec<String>>,

    /// Extra Volume Configuration
    #[arg(short, long, action = clap::ArgAction::Append)]
    pub environ: Option<Vec<String>>,

    /// Don't use the cache to build
    #[arg(short, long)]
    pub no_cache: bool,

    pub command: Option<String>,
}

fn find_composer() -> Result<PathBuf> {
    let composer = match which("docker-compose") {
        Ok(v) => v,
        Err(_) => which("podman-compose").context("podman-compose was not found")?,
    };

    Ok(composer)
}

fn find_compose(command: &str) -> Result<PathBuf> {
    let config_dir = get_config_dir()?;

    let command_dir = config_dir.join(command);

    if !command_dir.exists() {
        bail!("{} does not exist", command_dir.display());
    }

    let compose_file = command_dir.join("docker-compose.yml");

    if !compose_file.exists() {
        bail!("{} is missing", compose_file.display());
    }

    Ok(compose_file)
}

fn run_compose(user_args: &UserArgs, name: &str, compose_file: &Path) -> Result<()> {
    info!("{}", compose_file.display());

    let compose_dir = compose_file
        .parent()
        .ok_or(anyhow!("parent directory not found"))?;

    let composer = find_composer().context("Unable to find composer")?;

    info!("spawning {} {}", composer.display(), compose_file.display());

    let mut args = vec!["up"];

    if let Some(environs) = &user_args.environ {
        for env in environs.iter() {
            args.push("-e");
            args.push(env.as_str());
        }
    }

    if let Some(volumes) = &user_args.volume {
        for vol in volumes.iter() {
            args.push("-v");
            args.push(vol.as_str());
        }
    }

    let mut container = Command::new(composer)
        .current_dir(compose_dir)
        .args(args)
        .arg(name)
        .spawn()?;

    let result = container.wait()?;

    if let Some(code) = result.code() {
        info!("{name} returned {code}");
    }

    Ok(())
}

fn build_image(user_args: &UserArgs, compose_file: &Path) -> Result<()> {
    info!("{}", compose_file.display());

    let compose_dir = compose_file
        .parent()
        .ok_or(anyhow!("parent directory not found"))?;

    let composer = find_composer().context("Unable to find composer")?;

    info!("building {}", compose_file.display());

    let mut build_args = vec!["build"];

    if user_args.no_cache {
        build_args.push("--no-cache");
    }

    let mut builder = Command::new(composer)
        .current_dir(compose_dir)
        .args(build_args)
        .spawn()?;

    let result = builder.wait()?;

    if let Some(code) = result.code() {
        info!("returned {code}");
    }

    Ok(())
}

pub fn run_command(args: &UserArgs) -> Result<()> {
    if let Some(command) = &args.command {
        let compose_file = find_compose(command)?;

        build_image(&args, &compose_file)?;
        run_compose(&args, command, &compose_file)
    } else {
        bail!("Command not found")
    }
}
