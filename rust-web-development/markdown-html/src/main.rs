use markdown::to_html;
fn main() {
    let markdown = include_str!("../sample.md");
    let html = to_html(markdown);
    println!("{}", html);
}
