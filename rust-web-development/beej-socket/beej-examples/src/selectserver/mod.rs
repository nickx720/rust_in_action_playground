use std::net::{Ipv6Addr, SocketAddrV6};

pub fn selectserver(port: u16) {
    let unspec = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
    let socket: nix::sys::socket::SockaddrIn6 = unspec.into();
    let listener = nix::sys::socket::socket(
        nix::sys::socket::AddressFamily::Inet6,
        nix::sys::socket::SockType::Stream,
        nix::sys::socket::SockFlag::empty(),
        None,
    )
    .expect("Failed to create socket");
    nix::sys::socket::setsockopt(&listener, nix::sys::socket::sockopt::ReuseAddr, &true)
        .expect("Failed to set socket options");
}
