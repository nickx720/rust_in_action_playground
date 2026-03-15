use std::{env, fs};

#[path = "lib/mod.rs"]
mod bloom;

use crate::bloom::Bloom;

fn build_word_bf(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut bloom = Bloom::new(100, 0.01);
    let file = fs::read_to_string(path).expect("Reading file failed");
    for item in file.split("\n") {
        if item.is_empty() {
            continue;
        }
        bloom.insert(item);
    }
    fs::write("words.bf", bloom.save_to_disk())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        return Err("Needs a source of words to build".into());
    }
    if let Some(path) = args.get(1) {
        let file = fs::read("words.bf")?;
        let bloom = Bloom::read_from_disk(file);
        let file = fs::read_to_string(path).expect("Reading file failed");
        for item in file.split("\n") {
            if item.is_empty() {
                continue;
            }
            let output = bloom.exists(item);
        }
    }
    Ok(())
}
