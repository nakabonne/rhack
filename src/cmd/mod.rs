mod edit;
mod undo;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
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

#[derive(Debug, Clap)]
#[clap(
    about,
    author,
    global_setting(AppSettings::ColoredHelp),
    global_setting(AppSettings::GlobalVersion),
    global_setting(AppSettings::VersionlessSubcommands),
    version = "0.1.0"
)]
pub enum App {
    Edit(Edit),
    Undo(Undo),
}

impl Cmd for App {
    fn run(&self) -> Result<()> {
        match self {
            App::Edit(cmd) => cmd.run(),
            App::Undo(cmd) => cmd.run(),
        }
    }
}

// Gives back user-difined rhack dir path. If none, the default will be given.
pub fn rhack_dir() -> PathBuf {
    match env::var(RHACK_DIR_ENV_KEY) {
        Ok(path) => {
            return PathBuf::from(path);
        }
        Err(_) => {
            let home_dir = match home::home_dir() {
                Some(path) => path,
                None => panic!("failed to find home directory"),
            };
            return home_dir.join(DEFAULT_RHACK_DIR_NAME);
        }
    };
}

// Gives back the the path to Cargo.toml reffered from the working directory.
pub fn manifest_path() -> Result<String> {
    // Run "cargo locate-project" to find out Cargo.toml file's location.
    // See: https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html
    let out = Command::new("cargo").arg("locate-project").output();
    let out = match out {
        Ok(o) => o,
        Err(err) => return Err(anyhow!("failed to run \"cargo locate-project\": {:#}", err)),
    };
    let out: Value = serde_json::from_slice(&out.stdout)?;
    let path = match out["root"].as_str() {
        Some(p) => p,
        None => return Err(anyhow!("unexpected response from \"cargo locate-project\"")),
    };
    return Ok(path.to_string());
}

// Gives back the parsed Cargo.toml placed at the working directory.
pub fn load_manifest(manifest_path: &str) -> Result<Document> {
    let manifest = match fs::read_to_string(&manifest_path) {
        Ok(b) => b,
        Err(err) => return Err(anyhow!("failed to read from {}: {:#}", &manifest_path, err)),
    };
    return match manifest.parse::<Document>() {
        Ok(m) => Ok(m),
        Err(err) => Err(anyhow!("failed to parse {}: {:#}", &manifest_path, err)),
    };
}
