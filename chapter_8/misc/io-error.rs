use std::error::Error;
use std::fmt;
use std::fs::File;
use std::net::{AddrParseError, Ipv6Addr};

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for UpstreamError {}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt").map_err(UpstreamError::IO)?;

    let _localhost = ":1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}
