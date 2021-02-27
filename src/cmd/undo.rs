use super::Cmd;

use anyhow::{anyhow, Result};
use clap::Clap;

/// Undo everything
#[derive(Clap, Debug)]
pub struct Undo {}

impl Cmd for Undo {
    fn run(&self) -> Result<()> {
        return Err(anyhow!("undo command is not implemented"));
    }
}
