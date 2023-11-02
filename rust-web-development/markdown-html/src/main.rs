use markdown::to_html;
fn main() {
    let markdown = include_str!("../sample.md");

    println!("{}", to_html(markdown));
}
