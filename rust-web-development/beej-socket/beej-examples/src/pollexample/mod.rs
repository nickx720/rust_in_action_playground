use std::{
    net::{IpAddr, SocketAddrV4},
    os::fd::AsFd,
};

use nix::poll::{PollFd, PollFlags};

pub fn pollexample(host: IpAddr, port: u16) {
    match host {
        IpAddr::V4(addr) => {
            let socket = SocketAddrV4::new(addr, port);
            let socket: nix::sys::socket::SockaddrIn = socket.into();
            let sockfd = nix::sys::socket::socket(
                nix::sys::socket::AddressFamily::Inet,
                nix::sys::socket::SockType::Datagram,
                nix::sys::socket::SockFlag::empty(),
                None,
            )
            .expect("Failed to create sockfd");
            let pfd = PollFd::new(sockfd.as_fd(), PollFlags::POLLIN);
        }
        _ => panic!("not implemented"),
    }
}
