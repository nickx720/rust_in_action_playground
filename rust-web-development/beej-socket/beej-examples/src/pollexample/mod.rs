use std::{
    fs::File,
    os::fd::{AsFd, FromRawFd},
};

use nix::poll::{PollFd, PollFlags, PollTimeout};

pub fn pollexample() {
    println!("Hit return or wait 2.5 seconds");
    // TODO return on enter, listen for keyboard
    let file: File = unsafe { File::from_raw_fd(0) };
    let pfd = PollFd::new(file.as_fd(), PollFlags::POLLIN);
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
}
