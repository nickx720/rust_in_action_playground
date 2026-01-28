use std::io::{self, Read};

mod lib;
mod naive;
fn main() -> Result<(), anyhow::Error> {
    let mut input: Vec<u8> = vec![0u8; 512];
    let n = io::stdin().read(&mut input)?;
    // Read up to 512 bytes, then format for display: 16 bytes per line, grouped as 2-byte chunks with offsets; any line/grouping is just for readability, not a file "line".
    for (index, line) in input.chunks(16).enumerate() {
        let offset = index * 16;
        print!("{:08x}\t", offset);
        for group in line.chunks(2) {
            for item in group {
                print!("{:02x}", item);
            }
            print!(" ")
        }
        println!()
    }
    Ok(())
}
