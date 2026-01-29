use std::io::{self, Read};

mod lib;
mod naive;
fn main() -> Result<(), anyhow::Error> {
    let mut input: Vec<u8> = vec![0u8; 512];
    let mut block_offset = 0usize;
    loop {
        let n = io::stdin().read(&mut input)?;
        if n == 0 {
            break;
        }
        // Read up to 512 bytes, then format for display: 16 bytes per line, grouped as 2-byte chunks with offsets; any line/grouping is just for readability, not a file "line".
        let chunk = &input[..n];
        for (index, line) in chunk.chunks(16).enumerate() {
            let offset = block_offset * 512 + index * 16;
            print!("{:08x}\t", offset);
            for group in line.chunks(2) {
                for item in group {
                    print!("{:02x}", item);
                }
                print!(" ")
            }
            println!()
        }
        block_offset += 1;
    }
    Ok(())
}
