use clap::ValueEnum;
use std::fmt::Display;
#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Family {
    Ipv4,
    Ipv6,
    Unspecified,
}

impl Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Family::Ipv4 => write!(f, "ipv4"),
            Family::Ipv6 => write!(f, "ipv6"),
            Family::Unspecified => write!(f, "unspecified"),
        }
    }
}

impl From<Family> for libc::c_int {
    fn from(value: Family) -> Self {
        match value {
            Family::Ipv4 => libc::AF_INET,
            Family::Ipv6 => libc::AF_INET6,
            Family::Unspecified => libc::AF_UNSPEC,
        }
    }
}
impl From<libc::c_int> for Family {
    fn from(value: libc::c_int) -> Self {
        match value {
            libc::AF_INET => Family::Ipv4,
            libc::AF_INET6 => Family::Ipv6,
            _ => Family::Unspecified,
        }
    }
}
//impl Into<libc::c_int> for Family {
//    fn into(self) -> libc::c_int {
//        match self {
//            Family::Ipv4 => libc::AF_INET,
//            Family::Ipv6 => libc::AF_INET6,
//            Family::Unspecified => libc::AF_UNSPEC,
//        }
//    }
//}
#[derive(Debug, Clone, PartialEq)]
pub enum SocketType {
    Stream,
    Datagram,
}

impl From<SocketType> for libc::c_int {
    fn from(value: SocketType) -> Self {
        match value {
            SocketType::Stream => libc::SOCK_STREAM,
            SocketType::Datagram => libc::SOCK_DGRAM,
        }
    }
}

impl From<libc::c_int> for SocketType {
    fn from(value: libc::c_int) -> Self {
        match value {
            libc::SOCK_STREAM => SocketType::Stream,
            libc::SOCK_DGRAM => SocketType::Datagram,
            _ => panic!("unknown socket type"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Flag {
    /// No Flags
    None,
    Passive,
}

impl From<Flag> for libc::c_int {
    fn from(value: Flag) -> Self {
        match value {
            Flag::None => 0,
            Flag::Passive => libc::AI_PASSIVE,
        }
    }
}
impl From<libc::c_int> for Flag {
    fn from(value: libc::c_int) -> Self {
        match value {
            0 => Flag::None,
            libc::AI_PASSIVE => Flag::Passive,
            _ => panic!("unknown flag type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SockFd {
    Empty,
    Initialized(i32),
}

impl From<SockFd> for i32 {
    fn from(value: SockFd) -> Self {
        match value {
            SockFd::Empty => -1,
            SockFd::Initialized(fd) => fd,
        }
    }
}
