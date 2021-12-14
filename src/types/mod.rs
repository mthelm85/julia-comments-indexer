use std::path::PathBuf;

#[derive(Debug)]
pub struct Record {
    pub file: PathBuf,
    pub comments: Vec<String>
}
