use std::fmt;
use std::fs::File;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(std::net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for UpstreamError {}

impl From<std::io::Error> for UpstreamError {
    fn from(error: std::io::Error) -> Self {
        UpstreamError::IO(error)
    }
}

impl From<std::net::AddrParseError> for UpstreamError {
    fn from(error: std::net::AddrParseError) -> Self {
        UpstreamError::Parsing(error)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;

    let _localhost = ":1".parse::<Ipv6Addr>()?;

    Ok(())
}
