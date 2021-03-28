use super::{load_manifest, manifest_path, rhack_dir, Cmd, PATCH_TABLE_NAME, REGISTRY_TABLE_NAME};

use std::fs;
use std::path::PathBuf;

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
        let manifest = match load_manifest(&manifest_path) {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        if let Item::None = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME] {
            self.debug("patch section not found");
            return Ok(());
        }

        let table = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME]
            .as_table()
            .unwrap();

        let mut removed_crates = Vec::new();
        for item in table.iter() {
            let path = item.1["path"].as_value().unwrap().as_str().unwrap();
            let path = PathBuf::from(path);
            if path.starts_with(rhack_dir()) {
                removed_crates.push(item.0)
            }
        }

        // NOTE: To avoid being mutable borrow even though borrwoed as immutable once.
        let mut manifest = load_manifest(&manifest_path).unwrap();
        let table = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME]
            .as_table_mut()
            .unwrap();
        for c in removed_crates {
            println!("dropped {:?}", c);
            table.remove(c);
        }
        if table.is_empty() {
            table.set_implicit(true);
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
