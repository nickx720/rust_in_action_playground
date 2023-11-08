use std::fs;

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
            let path = path.path().into_os_string();
            let content = fs::read_to_string(path)?;
            let markdown = to_html(&content);
            println!("{}", markdown);
        }
    }
    Ok(())
}
