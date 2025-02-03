use std::{
    net::{IpAddr, Ipv4Addr, SocketAddrV4, SocketAddrV6, ToSocketAddrs},
    os::fd::AsRawFd,
    str::FromStr,
};

use nix::{ifaddrs::getifaddrs, net::if_::InterfaceFlags, unistd};

pub fn socketbroadcaster(host: IpAddr, port: u16, message: String) {
    match host {
        IpAddr::V4(addr) => {
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
            let host_addr = hostname.iter().nth(0).unwrap();
            let broadcast_addr = hostname.iter().nth(1).unwrap();
            let host_socket: nix::sys::socket::SockaddrIn =
                SocketAddrV4::new(Ipv4Addr::from_str(host_addr).unwrap(), port).into();
            let broadcast_addr: nix::sys::socket::SockaddrIn =
                SocketAddrV4::new(Ipv4Addr::from_str(broadcast_addr).unwrap(), 4950).into();
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
        IpAddr::V6(addr) => {
            let socket = SocketAddrV6::new(addr, port, 0, 0);
            todo!()
        }
    }
}
