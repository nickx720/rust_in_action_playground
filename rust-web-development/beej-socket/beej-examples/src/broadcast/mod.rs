use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};

use nix::{ifaddrs::getifaddrs, unistd};

pub fn socketbroadcaster(host: IpAddr, port: u16, message: String) {
    match host {
        IpAddr::V4(addr) => {
            let socket = SocketAddrV4::new(addr, port);
            let hostname = getifaddrs().ok().unwrap();
            for ifaddr in hostname {
                if let Some(address) = ifaddr.address {
                    // TODO try to print machine address
                    dbg!();
                }
            }
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
