use std::{
    net::{IpAddr, SocketAddrV4},
    os::fd::AsFd,
};

use nix::{
    poll::{PollFd, PollFlags, PollTimeout},
    unistd::pipe,
};

pub fn pollexample(host: IpAddr) {
    match host {
        IpAddr::V4(addr) => {
            let socket = SocketAddrV4::new(addr, 0);
            let (read_fd, write_fd) = pipe().expect("Something went wrong");
            println!("Hit return or wait 2.5 seconds");
            // TODO return on enter, listen for keyboard
            let pfd = PollFd::new(read_fd.as_fd(), PollFlags::POLLIN);
            let mut list_pfd: [PollFd; 1] = [pfd];
            let timeout = 2500u16;
            let timeout: PollTimeout = timeout.into();
            let num_events = nix::poll::poll(&mut list_pfd, timeout).expect("No num event created");
            if num_events == 0 {
                println!("Poll timeout");
            } else {
                let poll_event = pfd.revents().expect("Poll Flag not created");
                if poll_event.contains(PollFlags::POLLIN) {
                    println!("Yahoo");
                }
            }
            //            nix::unistd::close(read_fd).unwrap();
            //            nix::unistd::close(write_fd).unwrap();
        }
        _ => panic!("not implemented"),
    }
}
