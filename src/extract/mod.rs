use std::path::PathBuf;

use anyhow::Result;
use regex::Regex;

pub fn get_comments(path: PathBuf) -> Result<(PathBuf, Vec<String>)> {
    let single_line = Regex::new(r"#.*").unwrap();
    let multi_line = Regex::new(r"#=([\S\s]*?)=#").unwrap();

    let s = std::fs::read_to_string(&path)?;
    let mut comments: Vec<String> = Vec::new();
    for mat in single_line.find_iter(&s) {
        let matched = &s[mat.start() .. mat.end()];
        if matched.len() > 3 { comments.push(matched.to_string()) }
    }
    for mat in multi_line.find_iter(&s) {
        let matched = &s[mat.start() .. mat.end()];
        if matched.len() > 3 { comments.push(matched.to_string()) }
    }
    Ok((path, comments))
}