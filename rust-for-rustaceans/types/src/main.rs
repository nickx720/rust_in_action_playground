use std::{u32, u64, u8};

fn main() {
    let foo = Foo {
        tiny: false,
        normal: u32::MAX,
        small: u8::MAX,
        long: u64::MAX,
        short: u16::MAX,
    };
    dbg!("{}", std::mem::size_of::<Foo>());
}
// #[repr(align(1024))] 1024 bytes
// #[repr(packed)] 16 bytes
#[repr(C)] // 32 bytes with padding
#[derive(Debug)]
struct Foo {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}
