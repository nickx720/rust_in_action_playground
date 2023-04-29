#![feature(link_llvm_intrinsics)]
#![allow(non_camel_case_types)]
#![cfg(not(windows))]

use libc::{SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1};
use std::mem;

const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C" {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(_: *mut i8) -> i32;

    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(_: *mut i8);
}

#[inline]
fn ptr_to_jmp_buf() -> *mut i8 {
    unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jmp_buf();
    unsafe { longjmp(franken_pointer) }
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
    register_signal_handler();

    let should_shut_down = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };
    unsafe {
        SHUT_DOWN = should_shut_down;
    }

    return_early();
}
pub fn main_jmp() {}
