use std::fs;
use std::path::PathBuf;

mod extract;
mod search;
mod types;

use anyhow::Result;
use tantivy::schema::*;
use tantivy::{ doc, Index };

const JULIA_PATH: &str = "C:\\Users\\mthel\\Julia";

fn main() -> Result<()> {
    let index_path = dirs::data_local_dir().unwrap().join("JuliaSearch");
    fs::remove_dir_all(&index_path)?;
    fs::create_dir_all(&index_path)?;
    let jl_files = search::search(PathBuf::from(JULIA_PATH))?;
    let comments = jl_files.into_iter().map(extract::get_comments);
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("file", TEXT | STORED);
    schema_builder.add_text_field("comments", TEXT | STORED);
    let schema = schema_builder.build();
    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;
    let file = schema.get_field("file").unwrap();
    let file_comments = schema.get_field("comments").unwrap();

    comments
        .into_iter()
        .filter_map(|line| line.ok())
        .for_each(|c| {
            if !c.1.is_empty() && c.0.as_path().file_name().unwrap() != std::ffi::OsStr::new("notebook.jl")  {
                for cmt in c.1 {
                    index_writer.add_document(doc!(
                        file => c.0.to_str().unwrap(),
                        file_comments => cmt.trim().replace("\r\n", "")
                    ));
                }
            }
        });
    index_writer.commit()?;
    Ok(())
}
