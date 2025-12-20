use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result, anyhow, bail};
use log::info;
use which::which;

const CONFIG_DIR_NAME: &str = env!("CARGO_PKG_NAME");

fn find_compose(command: &str) -> Result<PathBuf> {
    let config_root = dirs::config_dir().ok_or(anyhow!("config dir not found"))?;

    let config_dir = config_root.join(CONFIG_DIR_NAME);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

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

fn run_compose(name: &str, compose_file: &Path) -> Result<()> {
    info!("{}", compose_file.display());

    let compose_dir = compose_file
        .parent()
        .ok_or(anyhow!("parent directory not found"))?;

    let composer = which("docker-compose")?;

    info!("spawning {}", compose_file.display());

    let mut container = Command::new(composer)
        .current_dir(compose_dir)
        .arg("run")
        .arg("--rm")
        .arg(name)
        .spawn()?;

    let result = container.wait()?;

    if let Some(code) = result.code() {
        info!("{name} returned {code}");
    }

    Ok(())
}

fn build_image(compose_file: &Path) -> Result<()> {
    info!("{}", compose_file.display());

    let compose_dir = compose_file
        .parent()
        .ok_or(anyhow!("parent directory not found"))?;

    let composer = which("docker-compose")?;

    info!("building {}", compose_file.display());

    let mut builder = Command::new(composer)
        .current_dir(compose_dir)
        .arg("build")
        .spawn()?;

    let result = builder.wait()?;

    if let Some(code) = result.code() {
        info!("returned {code}");
    }

    Ok(())
}

pub fn run_command(args: &[String]) -> Result<()> {
    let Some(name) = args.first() else {
        bail!("command not found")
    };

    let compose_file = find_compose(name)?;

    build_image(&compose_file)?;
    run_compose(name, &compose_file)
}
