use std::{env, fs};

fn valid_file_path(items: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
    for arg in items {
        let file = fs::canonicalize(arg)?;
    }
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1);
    valid_file_path(args)?;
    Ok(())
}
