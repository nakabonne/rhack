mod edit;
mod undo;

use anyhow::Result;
use clap::{AppSettings, Clap};

pub use edit::Edit;
pub use undo::Undo;

const DEFAULT_RHACK_DIR_NAME: &str = ".rhack";
const RHACK_DIR_ENV_KEY: &str = "RHACK_DIR";
const PATCH_TABLE_NAME: &str = "patch.'https://github.com/nakabonne/rhack'";

pub trait Cmd {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Clap)]
#[clap(
    about,
    author,
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
