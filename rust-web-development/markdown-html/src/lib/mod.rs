use std::{
    fmt::Display,
    fs::{self, File},
    path::PathBuf,
};
pub mod server;

use markdown::to_html;
use serde::Deserialize;
// read from a directory
// convert to markdown
// use wehooks for triggering generate
// http://danielwelch.github.io/rust-web-service.html
// make an api request to github directory
// read the contents
// convert to markdown
pub fn runfromlib(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir(path)?;
    for path in paths {
        if let Some(path) = path.ok() {
            if path.path().is_dir() {
                // If there are nested folders ignore and continue
                continue;
            }
            if let Some(extension) = path.path().extension().and_then(|value| value.to_str()) {
                match extension {
                    "md" => read_markdown_file(path.path())?,
                    "yaml" => read_yaml_config(path.path())?,
                    _ => panic!("File not supported"),
                }
            } else {
                panic!("Path not found")
            }
        }
    }
    Ok(())
}

fn read_markdown_file(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    println!("It is a markdown file");
    let markdown = to_html(&content);
    println!("{}", markdown);
    // Store this on s3
    Ok(())
}

#[derive(Deserialize)]
struct Pair {
    index: String,
}
impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The index file is {}", self.index)
    }
}

fn read_yaml_config(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let yaml: Pair = serde_yaml::from_reader(file)?;
    println!("{}", yaml);
    Ok(())
}
