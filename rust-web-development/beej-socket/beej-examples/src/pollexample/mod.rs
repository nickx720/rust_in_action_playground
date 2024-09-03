use std::{
    net::{IpAddr, SocketAddrV4},
    os::fd::AsFd,
    time::Duration,
};

use nix::poll::{PollFd, PollFlags, PollTimeout};

pub fn pollexample(host: IpAddr) {
    match host {
        IpAddr::V4(addr) => {
            let socket = SocketAddrV4::new(addr, 0);
            let socket: nix::sys::socket::SockaddrIn = socket.into();
            let sockfd = nix::sys::socket::socket(
                nix::sys::socket::AddressFamily::Inet,
                nix::sys::socket::SockType::Datagram,
                nix::sys::socket::SockFlag::empty(),
                None,
            )
            .expect("Failed to create sockfd");
            let pfd = PollFd::new(sockfd.as_fd(), PollFlags::POLLIN);
            let mut list_pfd: [PollFd; 1] = [pfd];
            let timeout = 2500u16;
            let timeout: PollTimeout = timeout.into();
            let num_events = nix::poll::poll(&mut list_pfd, timeout);
        }
        _ => panic!("not implemented"),
    }
}
