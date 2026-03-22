use std::{env, fs};

#[path = "lib/mod.rs"]
mod bloom;

use crate::bloom::{Bloom, BloomFilter};

fn build_word_bf(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut bloom = Bloom::new(100, 0.01);
    let path = fs::canonicalize(path)?;
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
    match args.as_slice() {
        [_] => Err("Needs a source of words to build".into()),
        [_, build, path] if *build == "build" => build_word_bf(path.clone()),
        items => {
            if items.is_empty() {
                return Err("Need words".into());
            }
            let file = fs::read("words.bf")?;
            let bloom = Bloom::read_from_disk(file);
            let mut print_output = Vec::new();
            for item in items {
                dbg!(item, bloom.exists(item));
                match bloom.exists(item) {
                    BloomFilter::MaybePresent => {
                        print_output.push(item);
                    }
                    _ => continue,
                }
            }
            let output: String = print_output
                .iter()
                .map(|&item| item.to_owned())
                .collect::<Vec<_>>()
                .join("\n");
            println!("These words are spelt wrong {}", output);
            Ok(())
        }
    }
    //    if args.len() < 1 {
    //        return;
    //    }
    //    if let Some(path) = args.get(1) {
    //        let file = fs::read("words.bf")?;
    //        let bloom = Bloom::read_from_disk(file);
    //        let file = fs::read_to_string(path).expect("Reading file failed");
    //        for item in file.split("\n") {
    //            if item.is_empty() {
    //                continue;
    //            }
    //            let output = bloom.exists(item);
    //        }
    //    }
}
