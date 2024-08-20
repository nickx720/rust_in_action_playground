use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use types::Family;

pub fn socketlistener(port: u16, family: Family) {
    let local_addr = match family {
        Family::Ipv4 => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)),
        Family::Ipv6 | Family::Unspecified => {
            SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0))
        }
    };
}
