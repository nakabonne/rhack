use super::{Cmd, DEFAULT_RHACK_DIR_NAME};

use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};
use clap::Clap;

/// Start hacking a crate
#[derive(Clap, Debug)]
pub struct Edit {
    crate_name: String,
}

impl Cmd for Edit {
    fn run(&self) -> Result<()> {
        // Copy the crate in registry to the rhack directory.
        let src = registry_path(&self.crate_name);
        let home_dir = match home::home_dir() {
            Some(path) => path,
            None => return Err(anyhow!("failed to find home directory")),
        };
        let dst = home_dir.join(DEFAULT_RHACK_DIR_NAME);
        let new_path = copy_dir(src, dst)?;

        update_manifest(new_path)
    }
}

fn registry_path(crate_name: &str) -> PathBuf {
    // FIXME: Give back real path
    return PathBuf::from(
        "/Users/nakabonne/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.1",
    );
}

// Copy the given src to the given dst. And then give back the path to newly created one.
fn copy_dir(src: PathBuf, dst: PathBuf) -> Result<PathBuf> {
    // FIXME: Mkdir if non-exixtence

    // TODO: Remove dependency on platform. Refer to gohack's implementation.
    let out = Command::new("cp").arg("-rf").arg(src).arg(dst).output();
    match out {
        Ok(_) => {
            // FIXME: Use the real path
            let path = PathBuf::from("/Users/nakabonne/.rhack/reqwest-0.11.1");
            Ok(path)
        }
        Err(err) => Err(anyhow!("failed to run cp command: {:#}", err)),
    }
}

//  Update [patch] section in Cargo.toml
fn update_manifest(new_path: PathBuf) -> Result<()> {
    // FIXME: Update [patch] section in Cargo.toml
    return Err(anyhow!("edit command is not implemented"));
}
