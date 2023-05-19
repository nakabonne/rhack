mod edit;
mod undo;

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use clap::Parser;
use serde_json::Value;
use toml_edit::Document;

pub use edit::Edit;
pub use undo::Undo;

const DEFAULT_RHACK_DIR_NAME: &str = ".rhack";
const RHACK_DIR_ENV_KEY: &str = "RHACK_DIR";
const PATCH_TABLE_NAME: &str = "patch";
const REGISTRY_TABLE_NAME: &str = "crates-io";

pub trait Cmd {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Parser)]
#[clap(about, author, version, propagate_version = true)]
pub enum App {
    Edit(Edit),
    Undo(Undo),
}

impl Cmd for App {
    fn run(&self) -> Result<()> {
        match self {
            Self::Edit(cmd) => cmd.run(),
            Self::Undo(cmd) => cmd.run(),
        }
    }
}

// Gives back user-difined rhack dir path. If none, the default will be given.
#[must_use]
pub fn rhack_dir() -> PathBuf {
    env::var(RHACK_DIR_ENV_KEY).map_or_else(
        |_| {
            let Some(home_dir) = home::home_dir() else {
                panic!("failed to find home directory")
            };
            home_dir.join(DEFAULT_RHACK_DIR_NAME)
        },
        PathBuf::from,
    )
}

// Gives back the the path to Cargo.toml reffered from the working directory.
pub fn manifest_path() -> Result<PathBuf> {
    // Run "cargo locate-project" to find out Cargo.toml file's location.
    // See: https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html
    let out = Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .output();

    let out = match out {
        Ok(o) => o,
        Err(err) => return Err(anyhow!("failed to run \"cargo locate-project\": {:#}", err)),
    };
    let out: Value = serde_json::from_slice(&out.stdout)?;
    let Ok(path) = PathBuf::from_str(match out["root"].as_str() {
        Some(it) => it,
        None => return Err(anyhow!("could convert to path")),
    }) else {
        return Err(anyhow!("unexpected response from \"cargo locate-project\""))
    };
    Ok(path)
}

// Gives back the parsed Cargo.toml placed at the working directory.
pub fn load_manifest(manifest_path: &PathBuf) -> Result<Document> {
    let manifest = match fs::read_to_string(manifest_path) {
        Ok(b) => b,
        Err(err) => {
            return Err(anyhow!(
                "failed to read from {}: {:#}",
                &manifest_path.display(),
                err
            ))
        }
    };
    match manifest.parse::<Document>() {
        Ok(m) => Ok(m),
        Err(err) => Err(anyhow!(
            "failed to parse {}: {:#}",
            &manifest_path.display(),
            err
        )),
    }
}
