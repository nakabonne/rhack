use super::{Cmd, DEFAULT_RHACK_DIR_NAME};

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
        let registry_path = registry_path(&self.crate_name);
        copy_dir(&registry_path, DEFAULT_RHACK_DIR_NAME)?;
        // FIXME: Update [patch] section in Cargo.toml
        return Err(anyhow!("edit command is not implemented"));
    }
}

fn registry_path(crate_name: &str) -> String {
    // FIXME: Give back real path
    return String::from(
        "/Users/nakabonne/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.1",
    );
}

fn copy_dir(src: &str, dst: &str) -> Result<()> {
    // FIXME: Mkdir if non-exixtence

    // TODO: Remove dependency on platform
    let out = Command::new("cp").arg("-rf").arg(src).arg(dst).output();
    match out {
        Ok(_) => {
            dbg!("success!");
            Ok(())
        }
        Err(err) => Err(anyhow!("failed to run cp command: {:#}", err)),
    }
}
