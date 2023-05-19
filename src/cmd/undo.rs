use super::{load_manifest, manifest_path, rhack_dir, Cmd, PATCH_TABLE_NAME, REGISTRY_TABLE_NAME};

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use toml_edit::{Item, Table};

/// Undo everything
#[derive(Parser, Debug)]
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

        if matches!(manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME], Item::None) {
            self.debug("patch section not found");
            return Ok(());
        }

        dbg!(&manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME]);

        let table = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME]
            .as_inline_table()
            .expect("parsing as table should not failed during 'undo'")
            .clone()
            .into_table();

        let mut removed_crates = Vec::new();

        for item in table.iter() {
            let path = item.1["path"].as_value().unwrap().as_str().unwrap();
            let path = PathBuf::from(path);
            if path.starts_with(rhack_dir()) {
                removed_crates.push(item.0);
            }
        }

        // NOTE: To avoid being mutable borrow even though borrowed as immutable once.
        let mut manifest = load_manifest(&manifest_path).unwrap();
        let table = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME]
            .as_inline_table_mut()
            .unwrap();

        for c in removed_crates {
            println!("dropped {c:?}");
            table.remove(c);
        }
        if table.is_empty() {
            // we probably can remove the whole table if it's empty
            manifest.remove_entry(PATCH_TABLE_NAME);
        }

        match fs::write(&manifest_path, manifest.to_string()) {
            Ok(_) => (),
            Err(err) => {
                return Err(anyhow!(
                    "failed to write to {}: {:#}",
                    &manifest_path.display(),
                    err
                ))
            }
        }

        Ok(())
    }
}

impl Undo {
    fn debug(&self, msg: &str) {
        if self.verbose {
            println!("{msg}");
        }
    }
}
