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
            if path.path().extension().unwrap() == "md" {
                read_markdown_file(path.path())?;
            } else {
                read_yaml_config(path.path())?;
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
    Ok(())
}

#[derive(Deserialize)]
struct Pair {
    key: String,
}
impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The key is {}", self.key)
    }
}

fn read_yaml_config(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let yaml: Pair = serde_yaml::from_reader(file)?;
    println!("{}", yaml);
    Ok(())
}
