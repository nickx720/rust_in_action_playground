use std::{
    net::{Ipv4Addr, SocketAddrV4},
    os::fd::AsRawFd,
    str::FromStr,
};

use nix::{ifaddrs::getifaddrs, net::if_::InterfaceFlags};

pub fn socketbroadcaster(port: u16, message: String) {
    let hostname: Vec<_> = getifaddrs()
        .ok()
        .unwrap()
        .filter(|ifaddr| {
            ifaddr.flags.contains(InterfaceFlags::IFF_LOOPBACK)
                || ifaddr.flags.contains(InterfaceFlags::IFF_BROADCAST)
        })
        .filter_map(|ifaddr| {
            ifaddr
                .address
                .and_then(|addr| addr.as_sockaddr_in().map(|inet| inet.ip().to_string()))
        })
        .collect();
    let broadcast_addr = hostname.get(1).expect("Not found");
    let broadcast_addr: nix::sys::socket::SockaddrIn =
        SocketAddrV4::new(Ipv4Addr::from_str(broadcast_addr).unwrap(), port).into();
    let sockfd = nix::sys::socket::socket(
        nix::sys::socket::AddressFamily::Inet,
        nix::sys::socket::SockType::Datagram,
        nix::sys::socket::SockFlag::empty(),
        None,
    )
    .expect("Failed to create sockfd");
    nix::sys::socket::sendto(
        sockfd.as_raw_fd(),
        message.as_bytes(),
        &broadcast_addr,
        nix::sys::socket::MsgFlags::empty(),
    )
    .expect("Failed to send message");
}
