use std::{
    net::{IpAddr, SocketAddrV4},
    os::fd::AsRawFd,
};

pub fn sockettalker(host: IpAddr, port: u16, message: String) {
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
            nix::sys::socket::sendto(
                sockfd.as_raw_fd(),
                message.as_bytes(),
                &socket,
                nix::sys::socket::MsgFlags::empty(),
            )
            .expect("Failed to send message");
        }
        IpAddr::V6(addr) => {}
    }
}
