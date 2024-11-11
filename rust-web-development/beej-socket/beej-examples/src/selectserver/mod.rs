use std::{
    net::{Ipv6Addr, SocketAddrV6},
    os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd},
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
        let active_fd: Vec<RawFd> = read_fds
            .fds(None)
            .map(|borrowed_fd| borrowed_fd.as_raw_fd())
            .collect();

        active_fd.iter().for_each(|fd| {
            if fd.as_raw_fd() == listener.as_raw_fd() {
                println!("[Server] Starting new connection...");
                let new_rfd = nix::sys::socket::accept(listener.as_raw_fd())
                    .expect("Failed to accept new conn");
                let new_fd = unsafe { BorrowedFd::borrow_raw(new_rfd) };
                let ss: nix::sys::socket::SockaddrStorage =
                    nix::sys::socket::getpeername(new_fd.as_raw_fd()).expect("getpeername failed");
                main.insert(new_fd);
                println!(
                    "[Server] New connection {}",
                    ss.as_sockaddr_in6().expect("sockaddr not ipv6")
                );
            } else {
            }
        });

        todo!()
    }
}
