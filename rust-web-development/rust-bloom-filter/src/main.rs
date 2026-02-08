use std::fs;

use crate::lib::Bloom;

mod lib;
fn main() {
    let mut bloom = Bloom::new(100, 0.01);
    let file = fs::read_to_string("./dict.txt").expect("Reading file failed");
    for item in file.split("\n") {
        if item.is_empty() {
            continue;
        }
        bloom.insert(item);
    }
}
