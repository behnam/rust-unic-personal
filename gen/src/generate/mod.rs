mod char_property;

mod ucd;

use std::fs;
use std::io;
use std::path::PathBuf;

/// The standard auto-generated source disclaimer.
const PREAMBLE: &'static str = "// WARNING: Auto-generated by unic-gen. DO NOT EDIT MANUALLY!\n";

/// Generate the sources for a UNIC subcrate
pub fn generate(package: &str) -> io::Result<()> {
    let path = tables_path(package);
    match package {
        "unic-ucd-core" => {
            if path.exists() {
                fs::remove_dir_all(&path)?;
            }
            fs::create_dir_all(&path)?;
            ucd::core::generate(path)?;
        },
        _ => println!("No files to generate for crate {}.", package),
    }
    Ok(())
}

/// Turn a crate name into the path to its tables folder
fn tables_path(package: &str) -> PathBuf {
    let mut path = PathBuf::new();
    for dir in package.split('-') {
        path.push(dir);
    }
    path.push("src/tables");
    path
}
