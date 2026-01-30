use std::io::{self, Read};

// Here’s the basic way to decode a UStar header (the first 512 bytes):

//  Key fields (byte offsets, length, encoding):
//
//  - 0..99 (100 bytes): name (null‑terminated string)
//  - 100..107 (8): mode (octal ASCII)
//  - 108..115 (8): uid (octal ASCII)
//  - 116..123 (8): gid (octal ASCII)
//  - 124..135 (12): size (octal ASCII, bytes)
//  - 136..147 (12): mtime (octal ASCII, Unix time)
//  - 148..155 (8): checksum (octal ASCII, space/NUL padded)
//  - 156 (1): typeflag (ASCII char, e.g. '0' regular file)
//  - 157..256 (100): linkname (string)
//  - 257..262 (6): magic = "ustar\0"
//  - 263..264 (2): version = "00"
//  - 265..296 (32): uname
//  - 297..328 (32): gname
//  - 329..336 (8): devmajor (octal)
//  - 337..344 (8): devminor (octal)
//  - 345..499 (155): prefix (path prefix for long names)
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
