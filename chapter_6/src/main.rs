mod references;
use references::{references, B, C};

use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

fn main() {
    //    references();
    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;

        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a : {}, b: {}, c: {}", a, b, c);
}
