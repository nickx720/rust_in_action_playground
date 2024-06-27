use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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
            Family::Unspecified => write!(f, "Unspecified"),
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

impl From<libc::c_int> for SocketType {
    fn from(value: libc::c_int) -> Self {
        match value {
            libc::SOCK_STREAM => SocketType::Stream,
            libc::SOCK_DGRAM => SocketType::Datagram,
            _ => panic!("Unknown socket type"),
        }
    }
}
