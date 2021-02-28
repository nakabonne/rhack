use super::{Cmd, DEFAULT_RHACK_DIR_NAME};

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};
use clap::Clap;
use serde::Deserialize;
use serde_json::Value;

/// Start hacking a crate
#[derive(Clap, Debug)]
pub struct Edit {
    crate_name: String,
}

impl Cmd for Edit {
    fn run(&self) -> Result<()> {
        // Copy the crate in registry to the rhack directory.
        let src = registry_path(&self.crate_name)?;
        let home_dir = match home::home_dir() {
            Some(path) => path,
            None => return Err(anyhow!("failed to find home directory")),
        };
        let dst = home_dir.join(DEFAULT_RHACK_DIR_NAME);
        let new_path = copy_dir(src, dst)?;

        update_manifest(&self.crate_name, new_path)
    }
}

// Gives back the local path to the directory holding the given crate.
fn registry_path(crate_name: &str) -> Result<PathBuf> {
    #[derive(Deserialize)]
    struct Metadata {
        packages: Vec<Package>,
    }

    #[derive(Deserialize)]
    struct Package {
        name: String,
        manifest_path: String,
    }

    let out = Command::new("cargo").arg("metadata").output();
    let out = match out {
        Ok(o) => o,
        Err(err) => return Err(anyhow!("failed to run \"cargo metadata\": {:#}", err)),
    };
    let metadata: Metadata = serde_json::from_slice(&out.stdout)?;

    let mut packages = HashMap::new();
    for p in metadata.packages {
        packages.insert(p.name, p.manifest_path);
    }
    let manifest_path = match packages.get(crate_name) {
        Some(m) => m,
        None => return Err(anyhow!("the given crate is not used in this project")),
    };
    let manifest_path = PathBuf::from(manifest_path);
    let path = manifest_path
        .parent()
        .expect("faild to determine the parent of manifest");
    return Ok(path.to_path_buf());
}

// Copy the given src to the given dst. And then give back the path to newly created one.
fn copy_dir(src: PathBuf, dst: PathBuf) -> Result<PathBuf> {
    // FIXME: Mkdir if non-existence

    // TODO: Remove dependency on platform. Refer to gohack's implementation.
    // Refer to: https://github.com/rogpeppe/gohack/blob/03d2ff3646b7ffc380e059413e4302f6cbdeb09b/io.go#L25-L47
    let out = Command::new("cp").arg("-rf").arg(src).arg(dst).output();
    match out {
        Ok(_) => {
            // FIXME: Give back the newly created one
            let path = PathBuf::from("dummy");
            Ok(path)
        }
        Err(err) => Err(anyhow!("failed to run cp command: {:#}", err)),
    }
}

//  Update [patch] section in Cargo.toml
fn update_manifest(crate_name: &str, new_path: PathBuf) -> Result<()> {
    let out = Command::new("cargo").arg("locate-project").output();
    let out = match out {
        Ok(o) => o,
        Err(err) => return Err(anyhow!("failed to run \"cargo locate-project\": {:#}", err)),
    };
    let out: Value = serde_json::from_slice(&out.stdout)?;
    let manifest_path = out["root"].as_str();

    // FIXME: Update [patch] section in Cargo.toml

    return Err(anyhow!("edit command is not implemented"));
}
