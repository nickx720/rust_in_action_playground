use std::{io, os::fd::AsFd};

use nix::poll::{PollFd, PollFlags, PollTimeout};

pub fn pollexample() {
    let file = io::stdin();
    let pfd = PollFd::new(file.as_fd(), PollFlags::POLLIN);
    let mut list_pfd: [PollFd; 1] = [pfd];
    let timeout = 2500u16;
    let timeout: PollTimeout = timeout.into();
    println!("Hit return or wait 2.5 seconds");
    let num_events = nix::poll::poll(&mut list_pfd, timeout).expect("No num event created");
    if num_events == 0 {
        println!("Poll timeout");
    } else {
        let poll_event = pfd.revents().expect("Poll Flag not created");
        let poll_check = poll_event & PollFlags::POLLIN;
        if poll_check.is_empty() {
            println!("We ready to read some data now")
        }
    }
}
