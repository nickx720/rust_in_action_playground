use std::{ffi::OsStr, fs};
pub mod server;

use markdown::to_html;
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
            // @TODO temporary value dropped why?
            if path.path().is_dir() {
                continue;
            }
            if path.path().extension().unwrap() == "md" {
                let content = fs::read_to_string(path.path())?;
                println!("It is a markdown file");
                let markdown = to_html(&content);
                println!("{}", markdown);
            } else {
                println!("{}", path.path().file_name().unwrap().to_str().unwrap());
            }
        }
    }
    Ok(())
}
