use std::{
    net::{Ipv6Addr, SocketAddrV6},
    os::fd::{AsFd, AsRawFd},
};

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

    nix::sys::socket::bind(listener.as_raw_fd(), &socket).expect("Failed to bind to socket");
    let backlog = nix::sys::socket::Backlog::new(10).expect("Failed to create backlog");
    nix::sys::socket::listen(&listener, backlog).expect("Failed to listen on socket");

    let mut main = nix::sys::select::FdSet::new();
    main.insert(listener.as_fd());

    println!("Listening on {}", unspec);
    loop {
        let mut read_fds = main.clone();
        let _ = nix::sys::select::select(None, Some(&mut read_fds), None, None, None)
            .expect("Failed to select...");
        todo!()
    }
}
