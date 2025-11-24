use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use flate2::bufread::GzDecoder;
use tar::{Archive, EntryType};

fn main() -> Result<(), anyhow::Error> {
    let file = File::open("test.tar.gz")?;
    let buf_reader = BufReader::new(file);
    let gz = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(gz);
    let entries = archive.entries()?;
    let root = PathBuf::from("output");
    if !fs::exists(&root)? {
        fs::create_dir(&root)?;
    }
    for entry in entries {
        let mut entry = entry?;
        let header = entry.header();
        if header.entry_type() == EntryType::Directory {
            let mut child_path = root.clone();
            child_path.push(header.path()?);
            if !fs::exists(&child_path)? {
                fs::create_dir(child_path)?;
            }
            continue;
        }
        entry.unpack(&root)?;
    }

    Ok(())
}
