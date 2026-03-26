use std::{fs, os::unix::fs::MetadataExt, path::Path};

use anyhow::anyhow;

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
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            name: [0u8; 100],
            mode: [0u8; 8],
            uid: [0u8; 8],
            gid: [0u8; 8],
            size: [0u8; 12],
            mtime: [0u8; 12],
            checksum: [0u8; 8],
            linkflag: [0u8; 1],
            linkname: [0u8; 100],
            pad: [0u8; 255],
        })
    }
    pub fn extract(buf: &[u8]) -> Result<Self, anyhow::Error> {
        //  - 0..99 (100 bytes): name (null-terminated string)
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
            return Err(anyhow!(name));
        }
        Ok(name.to_owned())
    }
    pub fn create_tar_header(path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        let mut name = [0u8; 100];
        if let Some(name_val) = path.file_name() {
            let bytes = name_val.as_encoded_bytes();
            let len = bytes.len().min(100);
            name[..len].copy_from_slice(&bytes[..len]);
        }
        let md = fs::symlink_metadata(&path)?;
        let mut mode_out = [0u8; 8];
        // drops file bits using mask
        let mode = (md.mode() & 0o7777) as u64;
        let s_mode = format!("{:07o}", mode);
        mode_out[..7].copy_from_slice(s_mode.as_bytes());

        let mut uid_out = [0u8; 8];
        let uid = md.uid() as u64;
        let s_uid = format!("{:07o}", uid);
        uid_out[..7].copy_from_slice(s_uid.as_bytes());

        let mut gid_out = [0u8; 8];
        let gid = md.gid() as u64;
        let s_gid = format!("{:07o}", gid);
        gid_out[..7].copy_from_slice(s_gid.as_bytes());

        let mut size_out = [0u8; 12];
        let size = md.size() as u64;
        let s_size = format!("{:011o}", size);
        size_out[0..11].copy_from_slice(s_size.as_bytes());

        let mut mtime_out = [0u8; 12];
        let mtime = md.mtime() as u64;
        let s_mtime = format!("{:011o}", mtime);
        mtime_out[0..11].copy_from_slice(s_mtime.as_bytes());

        let mut mlinkflag_out = [0u8; 1];
        let mlinkflag = md.file_type();
        let mlinkflag = if mlinkflag.is_symlink() {
            b'2'
        } else if mlinkflag.is_dir() {
            b'5'
        } else if mlinkflag.is_file() {
            b'0'
        } else {
            b'0'
        };
        mlinkflag_out[0..1].copy_from_slice(&[mlinkflag]);

        // linkname from read_link as_os_str.asencodedbytes
        let mut m_linkname_out = [0u8; 100];
        let linkname = fs::read_link(path)?;
        m_linkname_out[0..99].copy_from_slice(&linkname.as_os_str().as_encoded_bytes());

        let mut output = Vec::new();
        output.extend_from_slice(&name);
        output.extend_from_slice(&mode_out);
        output.extend_from_slice(&uid_out);
        output.extend_from_slice(&gid_out);
        output.extend_from_slice(&size_out);
        output.extend_from_slice(&mtime_out);
        output.extend_from_slice(&mlinkflag_out);
        output.extend_from_slice(&m_linkname_out);
        output.extend_from_slice(&[0u8; 255]);
        Ok(output)
    }
    pub fn create_body(path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        let contents = fs::read(path)?;
        if contents.len().is_multiple_of(512) {
            return Ok(contents);
        } else {
            // TODO: `offset = len % 512` is the right way to compute how much padding
            // the final partial tar block needs, but the mistake below is allocating
            // a brand new 512-byte `block` and trying to copy the whole file into it.
            // A tar body should keep all original file bytes and then append only the
            // trailing zero padding needed to reach the next 512-byte boundary.
            //
            // This `block[..contents.len()]` shape only works for files up to 512 bytes.
            // For files larger than that, like 700 or 1024 bytes, the body is still one
            // continuous byte stream of the full contents, not "one extra 512-byte block".
            let offset = contents.len() % 512;
            let padding = 512 - offset;
            let mut block = vec![0u8; 512];
            block[..contents.len()].copy_from_slice(&contents);
        }
        todo!()
    }
    pub fn create(path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        let header = TarHeader::create_tar_header(path)?;
        let body = TarHeader::create_body(path)?;
        Ok(header)
    }
}

impl TryFrom<&[u8]> for TarHeader {
    type Error = anyhow::Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 512 {
            anyhow::bail!("Buffer length too small");
        }
        Ok(Self::extract(buf)?)
    }
}
