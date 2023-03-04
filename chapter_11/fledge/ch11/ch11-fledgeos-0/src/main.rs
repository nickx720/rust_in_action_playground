#![feature(lang_items)]
#![no_std] // <1>
#![no_main] // <1>
#![feature(core_intrinsics)] // <2>

use core::intrinsics; // <2>
use core::panic::PanicInfo;

use x86_64::instructions::hlt; // <3>

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort(); // <4>
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0x0,
    White = 0xF,
    Blue = 0x1,
    BrightBlue = 0x9,
    Green = 0x2,
    BrightGreen = 0xA,
    Cyan = 0x3,
    BrightCyan = 0xB,
    Red = 0x4,
    BrightRed = 0xC,
    Magenta = 0x5,
    BrightMagenta = 0xD,
    Brown = 0x6,
    Yellow = 0xE,
    Gray = 0x7,
    DarkGray = 0x8,
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    unsafe {
        framebuffer
            .offset(1) // <5>
            .write_volatile(0x10); // <6>
    }

    loop {
        hlt();
    }
}
