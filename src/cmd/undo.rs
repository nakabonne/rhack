use super::{load_manifest, manifest_path, rhack_dir, Cmd, PATCH_TABLE_NAME, REGISTRY_TABLE_NAME};

use std::fs;

use anyhow::{anyhow, Result};
use clap::Clap;
use toml_edit::Item;

/// Undo everything
#[derive(Clap, Debug)]
pub struct Undo {
    /// Verbose output.
    #[clap(short, long)]
    verbose: bool,
}

impl Cmd for Undo {
    fn run(&self) -> Result<()> {
        let manifest_path = match manifest_path() {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        let mut manifest = match load_manifest(&manifest_path) {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        if let Item::None = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME] {
            self.debug("patch section not found");
            return Ok(());
        }

        let table = &manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME]
            .as_table()
            .unwrap();

        for item in table.iter() {
            // FIXME: Remove from the table if the path is underneath the rhack directory.
            // Using https://docs.rs/toml_edit/0.2.0/toml_edit/struct.Table.html#method.remove
            println!("{:?}", item);
        }

        match fs::write(&manifest_path, manifest.to_string_in_original_order()) {
            Ok(_) => (),
            Err(err) => return Err(anyhow!("failed to write to {}: {:#}", &manifest_path, err)),
        }

        return Ok(());
    }
}

impl Undo {
    fn debug(&self, msg: &str) {
        if self.verbose {
            println!("{}", msg);
        }
    }
}
