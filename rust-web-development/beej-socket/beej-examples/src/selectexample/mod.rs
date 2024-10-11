use std::{io, os::fd::AsFd};

pub fn selectexample(port: u16) {
    let tv = nix::sys::time::TimeVal::new(2, 500000);
    let mut fd_set = nix::sys::select::FdSet::new();
    fd_set.clear();
    let stdin = io::stdin();
    fd_set.insert(stdin.as_fd());
    todo!()
}
