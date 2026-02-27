use std::{
    fs::File,
    io::{self, Read, Write},
};

use crate::lib::TarHeader;
mod lib;

//  Key fields (byte offsets, length, encoding):
//
//  Decoding rules:
//
//  - Strings are null‑terminated within their fixed field.
//  - Numeric fields are ASCII octal. Trim NUL/space, parse as base‑8.
//  - size gives data length; data follows header, padded to 512.
//
//           struct header_old_tar {
//                   char name[100];
//                   char mode[8];
//                   char uid[8];
//                   char gid[8];
//                   char size[12];
//                   char mtime[12];
//                   char checksum[8];
//                   char linkflag[1];
//                   char linkname[100];
//                   char pad[255];
//           };
//
// - next_header_offset = current_header_offset + 512 + round_up(size, 512)
//  - round_up(size, 512) = ((size + 511) / 512) * 512
//
//  Where size is the file’s byte length parsed from the header (ASCII octal). That’s why “512 offset” only works
//  for the first header.
//  Octal ASCII means the number is stored as text characters that represent an octal (base‑8) number.

//  Example:
//
//  - Bytes: b"0000000644\0" are the ASCII characters 0 0 0 0 0 0 0 6 4 4
//  - Interpreted as octal: 0644
//  - Converted to decimal: 420
//
//  So for tar headers, you read the field as a string (trim spaces and NULs), then parse it using base‑8. In Rust
//  that’s u64::from_str_radix(s, 8).
fn round_up(size: usize) -> usize {
    let output = ((size + 511) / 512) * 512;
    output
}
fn extract_file() -> Result<(), anyhow::Error> {
    let mut input: Vec<u8> = vec![0u8; 512];
    let mut block_offset = 0usize;
    let mut next_header: usize = 0usize;
    let mut reader = io::stdin().lock();
    let mut size_global: usize = 0usize;
    let mut file_name = String::new();
    loop {
        let n = reader.read(&mut input)?;
        if n == 0 {
            break;
        }
        let offset = block_offset * 512;
        // Read up to 512 bytes, then format for display: 16 bytes per line, grouped as 2-byte chunks with offsets; any line/grouping is just for readability, not a file "line".
        let chunk = &input[..n];
        if next_header == offset {
            let header = TarHeader::try_from(chunk)?;
            let size = header.size()?;
            next_header = offset + 512 + round_up(size);
            if let Ok(name) = header.name() {
                println!("{}", name);
                file_name = name;
                size_global = size;
            }
        } else {
            let content = &chunk[0..size_global];
            let mut file = File::create(&file_name)?;
            file.write_all(content)?;
            //            let content = String::from_utf8(content.to_vec())?;
            //            println!("{}", content.trim());
        }
        block_offset += 1;
    }
    Ok(())
}

fn create_tar(args: impl Iterator) -> Result<(), anyhow::Error> {
    todo!()
}
fn main() -> Result<(), anyhow::Error> {
    let mut arguements = std::env::args().skip(1);
    if let Some(action) = arguements.nth(0) {
        match action.as_str() {
            "cf" => create_tar(arguements)?,
            "xf" => extract_file()?,
            _ => panic!("Not supported"),
        }
    } else {
        let _ = extract_file()?;
    }

    Ok(())
}
