use std::{
    env,
    fs::{copy, read_dir},
};

use anyhow::{bail, Context, Ok, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    List,
    Use { name: String },
}

fn main() -> Result<()> {
    let ssh_dir = format!(
        "{}/.ssh/",
        env::var("HOME").with_context(|| "Could not determine HOME folder")?
    );
    let args = App::parse();
    match args.command {
        Command::List => list_available_config_files(ssh_dir),
        Command::Use { name } => use_config(ssh_dir, name),
    }
}

fn use_config(path: String, matcher: String) -> Result<()> {
    copy(
        format!("{}{}", path, "config"),
        format!("{}{}", path, "config.backup"),
    )
    .context("Failed to create backup of current config")?;

    let entries = read_dir(&path).context("Failed to read SSH directory")?;

    let filtered: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().into_string().ok())
        .filter_map(|name| name)
        .filter(|name| name.starts_with("config.") && name["config.".len()..].contains(&matcher))
        .collect();

    match filtered.len() {
        0 => bail!("No matches found given the pattern: `{}`", matcher),
        n if n > 1 => bail!(
            "Multiple matches found given the pattern `{}`:\n{}",
            matcher,
            filtered.join(", ")
        ),
        _ => {
            let config_name = &filtered[0];

            copy(
                format!("{}{}", path, config_name),
                format!("{}config", path),
            )
            .context(format!("Failed to set config to {}", config_name))?;

            std::process::Command::new("pkill")
                .arg("ssh-agent")
                .output()
                .context("Failed to kill ssh-agent")?;

            std::process::Command::new("ssh-agent")
                .arg("-s")
                .output()
                .context("Failed to start ssh-agent")?;

            eprintln!("Now using config.{}", config_name);
            Ok(())
        }
    }
}

fn list_available_config_files(path: String) -> Result<()> {
    let entries = read_dir(path).unwrap();
    for entry in entries {
        if let Result::Ok(e) = entry {
            let name = e.file_name();
            let name = name.to_str().unwrap();
            if name.starts_with("config.") && !name.ends_with("backup") {
                println!("{} ", name.split(".").last().unwrap());
            }
        }
    }
    Ok(())
}
