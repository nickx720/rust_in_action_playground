use std::{fs::File, io::BufReader, path::Path};

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
        if let Ok(entry_path) = entry.path() {
            if Path::new(entry_path.as_os_str()).is_dir() {
                dbg!("has root", entry_path);
            }
        }
    }

    Ok(())
}
