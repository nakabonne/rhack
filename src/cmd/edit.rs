use super::{load_manifest, manifest_path, rhack_dir, Cmd, PATCH_TABLE_NAME, REGISTRY_TABLE_NAME};

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Result};
use clap::Parser;
use serde::Deserialize;
use toml_edit::{value, Item, Table};

/// Start hacking a crate
#[derive(Parser, Debug)]
pub struct Edit {
    crate_name: String,

    /// Verbose output.
    #[clap(short, long)]
    verbose: bool,
}

impl Cmd for Edit {
    fn run(&self) -> Result<()> {
        // Determine the destination directory and the source directory.
        let src = self.crate_local_path()?;
        let mut dst = PathBuf::from(src.file_name().unwrap());
        dst = rhack_dir().join(dst);

        match self.copy_dir(&src, &dst) {
            Ok(_) => (),
            Err(err) => return Err(anyhow!("failed to copy {:?} to {:?}: {}", src, dst, err)),
        }

        self.update_manifest(&dst)?;
        println!("{:?} => {:?}", &self.crate_name, dst);
        Ok(())
    }
}

impl Edit {
    // Gives back the local path to the directory holding the given crate.
    fn crate_local_path(&self) -> Result<PathBuf> {
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
        let manifest_path = match packages.get(&self.crate_name) {
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
    fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(
        &self,
        from: U,
        to: V,
    ) -> Result<(), std::io::Error> {
        let mut stack = Vec::new();
        stack.push(PathBuf::from(from.as_ref()));

        // TODO: Delete dir if the destination dir already exists.
        let output_root = PathBuf::from(to.as_ref());
        let input_root = PathBuf::from(from.as_ref()).components().count();

        while let Some(working_path) = stack.pop() {
            self.debug(&format!("\nprocess: {:?}", &working_path));

            // Generate a relative path
            let src: PathBuf = working_path.components().skip(input_root).collect();

            // Create a destination if missing
            let dest = if src.components().count() == 0 {
                output_root.clone()
            } else {
                output_root.join(&src)
            };
            if fs::metadata(&dest).is_err() {
                self.debug(&format!("  mkdir: {:?}", dest));
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
                            self.debug(&format!("    copy: {:?} -> {:?}", &path, &dest_path));
                            fs::copy(&path, &dest_path)?;
                        }
                        None => {
                            println!("failed to copy: {:?}", path);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    //  Update [patch.crates-io] section in Cargo.toml
    fn update_manifest(&self, new_path: &PathBuf) -> Result<()> {
        let manifest_path = match manifest_path() {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        let mut manifest = match load_manifest(&manifest_path) {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        // Insert [patch.crates-io] table if it doesn't exist.
        if let Item::None = manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME] {
            let mut t = Table::new();
            t.set_implicit(true);
            manifest[PATCH_TABLE_NAME] = Item::Table(t);
            manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME] = Item::Table(Table::new());
        }
        manifest[PATCH_TABLE_NAME][REGISTRY_TABLE_NAME][&self.crate_name]["path"] =
            value(new_path.to_str().unwrap());

        match fs::write(&manifest_path, manifest.to_string_in_original_order()) {
            Ok(_) => (),
            Err(err) => return Err(anyhow!("failed to write to {}: {:#}", &manifest_path, err)),
        }

        return Ok(());
    }

    fn debug(&self, msg: &str) {
        if self.verbose {
            println!("{}", msg);
        }
    }
}
