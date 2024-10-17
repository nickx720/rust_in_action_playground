use std::{
    io,
    os::fd::{AsFd, AsRawFd},
};

pub fn selectexample() {
    let mut tv = nix::sys::time::TimeVal::new(2, 500000);
    let mut fd_set = nix::sys::select::FdSet::new();
    fd_set.clear();
    let stdin = io::stdin();
    fd_set.insert(stdin.as_fd());
    let _ = nix::sys::select::select(stdin.as_raw_fd() + 1, &mut fd_set, None, None, &mut tv);
    if fd_set.contains(stdin.as_fd()) {
        println!("A key was pressed\n");
    } else {
        println!("Nothing happened")
    }
}
