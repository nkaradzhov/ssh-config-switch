use std::{
    env,
    fs::{copy, read_dir},
};

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

fn main() {
    let ssh_dir = format!("{}/.ssh/", env::var("HOME").unwrap());
    let args = App::parse();
    match args.command {
        Command::List => list_available_config_files(ssh_dir),
        Command::Use { name } => use_config(ssh_dir, name),
    }
}

fn use_config(path: String, config_name: String) {
    copy(
        format!("{}{}", path, "config"),
        format!("{}{}", path, "config.backup"),
    )
    .unwrap();
    let entries = read_dir(&path).unwrap();
    let mut found = false;
    for entry in entries {
        if let Ok(e) = entry {
            let name = e.file_name();
            let name = name.to_str().unwrap();
            if name == format!("config.{}", config_name) {
                found = true;
                copy(
                    format!("{}config.{}", path, config_name),
                    format!("{}config", path),
                )
                .unwrap();

                std::process::Command::new("pkill")
                    .arg("ssh-agent")
                    .output()
                    .expect("Failed to kill ssh-agent");

                std::process::Command::new("ssh-agent")
                    .arg("-s")
                    .output()
                    .expect("Failed to start ssh-agent");

                eprintln!("Now using config.{}", config_name);
            }
        }
    }
    if !found {
        eprintln!("Could not find config named: {}", config_name);
        std::process::exit(1);
    }
}

fn list_available_config_files(path: String) {
    let entries = read_dir(path).unwrap();
    for entry in entries {
        if let Ok(e) = entry {
            let name = e.file_name();
            let name = name.to_str().unwrap();
            if name.starts_with("config.") && !name.ends_with("backup") {
                println!("{} ", name.split(".").last().unwrap());
            }
        }
    }
}
