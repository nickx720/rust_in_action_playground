use markdown::to_html;
pub fn runfromlib(path: String) {
    let path = path.to_owned();
    // https://stackoverflow.com/questions/68865499/include-str-set-string-literal-path
    let markdown = include_str!(path);
    let html = to_html(markdown);
    println!("{}", html);
}
