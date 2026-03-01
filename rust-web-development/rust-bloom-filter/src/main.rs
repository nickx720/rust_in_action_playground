use std::{env, fs};

#[path = "lib/mod.rs"]
mod bloom;

use crate::bloom::Bloom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Needs a source of words to build".into());
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
        fs::write("words.bf", bloom.save_to_disk())?;
        let file = fs::read("words.bf")?;
        let bloom = Bloom::read_from_disk(file);
    }
    Ok(())
}
