use std::net::{IpAddr, SocketAddrV4, SocketAddrV6};

use nix::{ifaddrs::getifaddrs, net::if_::InterfaceFlags, unistd};

pub fn socketbroadcaster(host: IpAddr, port: u16, message: String) {
    match host {
        IpAddr::V4(addr) => {
            let socket = SocketAddrV4::new(addr, port);
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
            dbg!(hostname);
            let socket: nix::sys::socket::SockaddrIn = socket.into();
            let sockfd = nix::sys::socket::socket(
                nix::sys::socket::AddressFamily::Inet,
                nix::sys::socket::SockType::Datagram,
                nix::sys::socket::SockFlag::empty(),
                None,
            )
            .expect("Failed to create sockfd");
            if nix::sys::socket::setsockopt(&sockfd, nix::sys::socket::sockopt::Broadcast, &true)
                .is_err()
            {
                panic!("Socket broadcast failed");
            }
        }
        IpAddr::V6(addr) => {
            let socket = SocketAddrV6::new(addr, port, 0, 0);
        }
    }
    todo!()
}
