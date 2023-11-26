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
            let extension = path.path().extension().and_then(OsStr::to_str);
            let path = path.path().into_os_string();
            if extension.is_none() {
                let content = fs::read_to_string(path)?;
                println!("It is a markdown file");
                let markdown = to_html(&content);
                println!("{}", markdown);
            } else {
                println!("What sort of file is this?");
            }
        }
    }
    Ok(())
}
