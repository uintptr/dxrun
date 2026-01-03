use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow, bail};
use clap::Parser;
use log::info;
use tokio::{fs, process::Command, select};
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

    #[arg(trailing_var_arg = true)]
    pub extras: Vec<String>,
}

async fn exports_service(compose_file: &Path) -> Result<bool> {
    let data = fs::read_to_string(compose_file).await?;
    Ok(data.contains("ports:"))
}

fn find_compose(command: &str) -> Result<PathBuf> {
    let config_dir = get_config_dir()?;

    let command_dir = config_dir.join(command);

    if !command_dir.exists() {
        bail!("{} does not exist", command_dir.display());
    }

    //
    // make sure the directory contains both the docker and compose files
    //
    let docker_file = command_dir.join("Dockerfile");
    let compose_file = command_dir.join("docker-compose.yml");

    if !docker_file.exists() {
        bail!("{} is missing", docker_file.display());
    }
    if !compose_file.exists() {
        bail!("{} is missing", compose_file.display());
    }

    Ok(compose_file)
}

async fn run_compose(user_args: &UserArgs, name: &str, compose_file: &Path) -> Result<()> {
    info!("{}", compose_file.display());

    let compose_dir = compose_file
        .parent()
        .ok_or(anyhow!("parent directory not found"))?;

    let composer = which("docker-compose")?;

    info!("spawning {}", compose_file.display());

    let mut args = vec!["run", "--rm"];

    if exports_service(&compose_file).await? {
        args.push("--service-ports");
    }

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

    let result = select! {
        res = container.wait() => res?,
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, terminating container");
            container.kill().await?;
            bail!("Interrupted by user");
        }
    };

    if let Some(code) = result.code() {
        info!("{name} returned {code}");
    }

    Ok(())
}

async fn build_image(user_args: &UserArgs, compose_file: &Path) -> Result<()> {
    info!("{}", compose_file.display());

    let compose_dir = compose_file
        .parent()
        .ok_or(anyhow!("parent directory not found"))?;

    let composer = which("docker-compose")?;

    info!("building {}", compose_file.display());

    let mut build_args = vec!["build"];

    if user_args.no_cache {
        build_args.push("--no-cache");
    }

    let mut builder = Command::new(composer)
        .current_dir(compose_dir)
        .args(build_args)
        .spawn()?;

    let result = builder.wait().await?;

    if let Some(code) = result.code() {
        info!("docker-compose build returned {code}");

        if 0 != code {
            bail!("container build failure");
        }
    }

    Ok(())
}

pub async fn run_command(args: &UserArgs) -> Result<()> {
    let command = args.extras.first().ok_or(anyhow!("missing command"))?;

    let compose_file = find_compose(command)?;

    build_image(&args, &compose_file).await?;
    run_compose(&args, command, &compose_file).await
}
