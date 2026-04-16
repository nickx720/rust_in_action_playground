use std::{
    env,
    fs::{self, File},
    io::Read,
};

fn frequency_counter(data: &[u8]) -> Result<(), anyhow::Error> {
    let string_to_count = String::from_utf8_lossy(data);
    for word in string_to_count.trim().split("") {
        dbg!(word);
    }
    todo!();
}

fn valid_file_path(items: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
    for arg in items {
        let file = fs::canonicalize(arg)?;
        let mut file = File::open(file)?;
        let mut buf = [0u8; 1024];
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            let data = &buf[..n];
            frequency_counter(data)?;
        }
    }
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1);
    valid_file_path(args)?;
    Ok(())
}
