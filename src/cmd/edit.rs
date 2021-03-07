use super::{Cmd, DEFAULT_RHACK_DIR_NAME, RHACK_DIR_ENV_KEY};

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Result};
use clap::Clap;
use serde::Deserialize;
use serde_json::Value;

/// Start hacking a crate
#[derive(Clap, Debug)]
pub struct Edit {
    crate_name: String,

    /// Verbose output.
    #[clap(short, long)]
    verbose: bool,
}

impl Cmd for Edit {
    fn run(&self) -> Result<()> {
        // Determine the destination directory and the source directory.
        let src = registry_path(&self.crate_name)?;
        let mut dst = PathBuf::from(src.file_name().unwrap());
        match env::var(RHACK_DIR_ENV_KEY) {
            Ok(v) => {
                dst = PathBuf::from(v).join(dst);
            }
            Err(err) => {
                let home_dir = match home::home_dir() {
                    Some(path) => path,
                    None => return Err(anyhow!("failed to find home directory: {}", err)),
                };
                dst = home_dir.join(DEFAULT_RHACK_DIR_NAME).join(dst);
            }
        };

        match copy_dir(&src, &dst) {
            Ok(_) => (),
            Err(err) => return Err(anyhow!("failed to copy {:?} to {:?}: {}", src, dst, err)),
        }

        update_manifest(&self.crate_name, &dst)?;
        println!("{:?} => {:?}", &self.crate_name, dst);
        Ok(())
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

// Copy the given src to the given dst recursively.
fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // FIXME: emit only in verbose mode
        println!("");
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!("  mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("    copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

//  Update [patch] section in Cargo.toml
fn update_manifest(crate_name: &str, new_path: &PathBuf) -> Result<()> {
    // Run "cargo locate-project" to find out Cargo.toml file's location.
    // See: https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html
    let out = Command::new("cargo").arg("locate-project").output();
    let out = match out {
        Ok(o) => o,
        Err(err) => return Err(anyhow!("failed to run \"cargo locate-project\": {:#}", err)),
    };
    let out: Value = serde_json::from_slice(&out.stdout)?;
    let manifest_path = out["root"].as_str();

    // FIXME: Update [patch] section in Cargo.toml
    // FIXME: Consider using [patch.rhack] instead of [patch.crates-io]

    return Err(anyhow!("edit command is not implemented"));
}
