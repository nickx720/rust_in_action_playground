use std::fs;

use markdown::to_html;
pub fn runfromlib(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir(path)?;
    for path in paths {
        if let Some(path) = path.ok() {
            let path = path.path().into_os_string();
            let content = fs::read_to_string(path)?;
            println!("{}", content);
            //println!("Name of the file {}", path.path().display());
        }
    }
    // https://stackoverflow.com/questions/68865499/include-str-set-string-literal-path
    //  let markdown = include_str!(path);
    //  let html = to_html(markdown);
    //  println!("{}", html);
    Ok(())
}
