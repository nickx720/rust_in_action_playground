use std::{env, fs};

fn frequency_counter(contents: Vec<u8>) -> Result<(), anyhow::Error> {
    dbg!(std::str::from_utf8(&contents));
    todo!();
}

fn valid_file_path(items: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
    for arg in items {
        let file = fs::canonicalize(arg)?;
        let contents = fs::read(file)?;
        frequency_counter(contents)?;
    }
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1);
    valid_file_path(args)?;
    Ok(())
}
