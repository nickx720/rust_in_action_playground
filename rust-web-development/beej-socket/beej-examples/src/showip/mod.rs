use builders::AddrInfo;
use nix::libc;
use std::{ffi::CString, ptr};

use types::Family;

pub fn show_ip(host: String, family: Family, service: String) -> i32 {
    println!("IP addresses for {}\n", host);
    let host = CString::new(host).expect("Invalid host");
    let c_host: *const libc::c_char = host.as_ptr() as *const libc::c_char;

    let service =
        CString::new(service).expect("Invalid service, service should map to port number");
    let service: *const libc::c_char = service.as_ptr() as *const libc::c_char;
    let addrinfo = AddrInfo::builder().family(family).build();
    let hints: AddrInfo = addrinfo.into();
    let mut res = ptr::null_mut();
    // TODO fix the type mismatch
    unsafe { libc::getaddrinfo(c_host, service, &hints, &mut res) };
    todo!()
}
