use super::Cmd;

use anyhow::{anyhow, Result};
use clap::Clap;

/// Start hacking a crate
#[derive(Clap, Debug)]
pub struct Edit {}

impl Cmd for Edit {
    fn run(&self) -> Result<()> {
        return Err(anyhow!("edit command is not implemented"));
    }
}
