use std::{
    net::{Ipv6Addr, SocketAddrV6},
    os::fd::{AsFd, AsRawFd, BorrowedFd},
};

pub fn pollserver(port: u16) {
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

    let listener_pfd = nix::poll::PollFd::new(listener.as_fd(), nix::poll::PollFlags::POLLIN);
    let mut pfds = Vec::with_capacity(5);
    pfds.push(listener_pfd);
    println!("Listening on {}", unspec);

    loop {
        println!("polling for events, we have '{}' poll fds", pfds.len());

        let num_events =
            nix::poll::poll(&mut pfds, nix::poll::PollTimeout::NONE).expect("poll failed");

        if num_events > 0 {
            println!("Events ready: {}", num_events);
        }
        for i in 0..pfds.len() {
            let pfd = pfds[i];
            if let Some(e) = pfd.revents() {
                if e.contains(nix::poll::PollFlags::POLLIN) {
                    if pfd.as_fd().as_raw_fd() == listener.as_raw_fd() {
                        println!("[New Connection] Attaching to poll list");
                        let new_fd = nix::sys::socket::accept(pfd.as_fd().as_raw_fd())
                            .expect("Failed to accept new connection");
                        let new_bfd = unsafe { BorrowedFd::borrow_raw(new_fd) };
                        let new_pfd = nix::poll::PollFd::new(new_bfd, nix::poll::PollFlags::POLLIN);
                        pfds.push(new_pfd);
                        // convert fd to address
                        let ss: nix::sys::socket::SockaddrStorage =
                            nix::sys::socket::getpeername(new_fd).expect("getpeername failed");
                        println!("New connection, {ss:?}");
                    } else {
                        println!("[Client] preparing to read...");
                        let mut buf = [0u8; 1024];
                        let nbytes = nix::sys::socket::recv(
                            pfd.as_fd().as_raw_fd(),
                            &mut buf,
                            nix::sys::socket::MsgFlags::empty(),
                        )
                        .expect("recv failed");
                        println!("[Client] Received {} bytes", nbytes);
                        // TODO nbytes
                        if nbytes == 0 {
                            println!("[Client] Connection closed");
                            pfds.remove(i);
                        } else {
                            println!("[Client] Reading bytes...");
                            // Send data to all clients but the sender and listener
                            for (j, pfd) in pfds.iter().skip(1).enumerate() {
                                if i != j {
                                    let ss: nix::sys::socket::SockaddrStorage =
                                        nix::sys::socket::getpeername(pfd.as_fd().as_raw_fd())
                                            .expect("getpeername failed");
                                    nix::sys::socket::sendto(
                                        pfd.as_fd().as_raw_fd(),
                                        &buf[..nbytes],
                                        &ss,
                                        nix::sys::socket::MsgFlags::empty(),
                                    )
                                    .expect("sendto failed");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
