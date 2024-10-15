use std::{
    io,
    os::fd::{AsFd, AsRawFd},
};

pub fn selectexample(port: u16) {
    let mut tv = nix::sys::time::TimeVal::new(2, 500000);
    let mut fd_set = nix::sys::select::FdSet::new();
    fd_set.clear();
    let stdin = io::stdin();
    fd_set.insert(stdin.as_fd());
    let _ = nix::sys::select::select(stdin.as_raw_fd(), &mut fd_set, None, None, &mut tv);
    todo!()
}
