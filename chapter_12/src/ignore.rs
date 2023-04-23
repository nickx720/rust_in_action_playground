use libc::{raise, signal};
use libc::{SIGTERM, SIG_DFL, SIG_IGN};

pub fn main_ignore() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }
    println!("ok");
    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }
    println!("not ok");
}
