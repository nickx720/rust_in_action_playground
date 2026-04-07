use std::env;

fn valid_file_path(items: impl Iterator<Item = String>) {
    for arg in items {
        println!("{}", arg);
    }
    todo!()
}
fn main() {
    let args = env::args().skip(1);
    valid_file_path(args);
}
