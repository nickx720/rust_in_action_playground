#![cfg(not(windows))]

use libc::{SIGTERM, SIGUSR1};
use std::thread::sleep;
use std::time::Duration;

static mut SHUT_DOWN: bool = false;
pub fn shutdown_main() {
    register_signal_handlers();

    let delay = Duration::from_secs(1);
    for i in 1_usize {
        //todo
    }
}
