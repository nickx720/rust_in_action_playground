use std::{env, fs};

use crate::lib::Bloom;

mod lib;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Needs a source of words to build");
        return;
    }
    if let Some(path) = args.get(1) {
        let mut bloom = Bloom::new(100, 0.01);
        let file = fs::read_to_string(path).expect("Reading file failed");
        for item in file.split("\n") {
            if item.is_empty() {
                continue;
            }
            bloom.insert(item);
        }
        fs::write("words.bf", bloom.save_to_disk());
    }
}
