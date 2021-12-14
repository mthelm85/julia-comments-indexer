use std::path::PathBuf;

use anyhow::Result;
use walkdir::WalkDir;

pub fn search(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut jl_files: Vec<PathBuf> = Vec::new();
    let walker = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path().to_string_lossy();
        if path.ends_with(".jl") {
            jl_files.push(entry.into_path())
        }
    }
    Ok(jl_files)
}
