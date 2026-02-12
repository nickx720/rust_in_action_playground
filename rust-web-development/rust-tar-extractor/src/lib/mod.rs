#[derive(Debug)]
pub struct TarHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    checksum: [u8; 8],
    linkflag: [u8; 1],
    linkname: [u8; 100],
    pad: [u8; 255],
}

impl TarHeader {
    pub fn new(buf: &[u8]) -> Result<Self, anyhow::Error> {
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
        let name: [u8; 100] = buf.get(0..=99).unwrap().try_into()?;
        let mode: [u8; 8] = buf.get(100..=107).unwrap().try_into()?;
        let uid: [u8; 8] = buf.get(108..=115).unwrap().try_into()?;
        let gid: [u8; 8] = buf.get(116..=123).unwrap().try_into()?;
        let size: [u8; 12] = buf.get(124..=135).unwrap().try_into()?;
        let mtime: [u8; 12] = buf.get(136..=147).unwrap().try_into()?;
        let checksum: [u8; 8] = buf.get(148..=155).unwrap().try_into()?;
        let linkflag: [u8; 1] = buf.get(156..=156).unwrap().try_into()?;
        let linkname: [u8; 100] = buf.get(157..=256).unwrap().try_into()?;
        let pad: [u8; 255] = buf.get(257..=511).unwrap().try_into()?;
        Ok(Self {
            name,
            mode,
            uid,
            gid,
            size,
            mtime,
            checksum,
            linkflag,
            linkname,
            pad,
        })
    }
    pub fn size(&self) -> Result<usize, anyhow::Error> {
        let size = std::str::from_utf8(&self.size)?.trim_matches('\0').trim();
        if size.is_empty() {
            return Ok(0);
        }
        // Numeric fields are ASCII octal strings: trim NUL/space on the bytes, then parse base-8.
        // Example: b"0000000101\0" -> "0000000101" -> from_str_radix(_, 8) == 65.
        // NUL is not printable: if you print as a string you won't see it.
        // To see the raw bytes (including 00), try:
        //   println!("{:?}", &header.size);
        //   println!("{:02x?}", &header.size);
        // since it has nul terminator i am eliminating with trim
        let size = usize::from_str_radix(size.trim(), 8)?;
        Ok(size)
    }
    pub fn name(&self) -> Result<String, anyhow::Error> {
        let name = std::str::from_utf8(&self.name)?;
        let name = name.trim_end_matches('\0');
        if name.is_empty() {
            let name = "Nothing set";
            return Ok(name.to_owned());
        }
        Ok(name.to_owned())
    }
}

impl TryFrom<&[u8]> for TarHeader {
    type Error = anyhow::Error;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 512 {
            anyhow::bail!("Buffer length too small");
        }
        Ok(Self::new(buf)?)
    }
}
