use std::{env, fs};

#[path = "lib/mod.rs"]
mod bloom;

use crate::bloom::{Bloom, BloomFilter};

fn build_word_bf(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut bloom = Bloom::new(300000, 0.01);
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
    // TODO: tighten the query path so CLI behavior matches Bloom filter semantics.
    // Next concrete steps:
    // 1) Separate the executable name from the user-supplied words before querying.
    //    Hint: `env::args().skip(1)` or slicing `args[1..]` avoids checking the binary path.
    // 2) Revisit the meaning of `MaybePresent` vs `NotPresent` in the printed output.
    //    Hint: in a Bloom filter spell-checker, "maybe present" usually means "probably spelled correctly".
    // 3) Make the printed label match the branch you collect into `print_output`.
    //    Suggestion: decide whether you want to print probable matches or definite misses, then align both
    //    the `match` arm and the human-readable message with that choice.
    // 4) Add one focused CLI test case or temporary debug run that proves the program is checking only the
    //    words the user typed, not the executable path.
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
