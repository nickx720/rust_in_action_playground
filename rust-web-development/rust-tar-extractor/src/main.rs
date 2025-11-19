use std::{fs::File, io::BufReader};

use flate2::bufread::GzDecoder;
use tar::Archive;

fn main() -> Result<(), anyhow::Error> {
    let file = File::open("test.tar.gz")?;
    let buf_reader = BufReader::new(file);
    let gz = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(gz);
    let entries = archive.entries()?;
    for entry in entries {
        let entry = entry?;
        dbg!(entry.path());
    }

    Ok(())
}
