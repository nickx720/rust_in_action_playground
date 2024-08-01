use std::{
    ffi::{CStr, CString},
    mem, ptr,
};

use builders::AddrInfo;
use socket2::SockAddr;
use types::Family;

/// Simple Stream Server
pub fn streamserver() {
    let family = Family::Unspecified;
    let host = "localhost";
    let service = "3490";

    let addrinfo = AddrInfo::builder()
        .family(family)
        .flags(types::Flag::Passive)
        .build();
    let hints: libc::addrinfo = addrinfo.into();

    let mut servinfo = ptr::null_mut();
    let rv = unsafe {
        let node = CString::new(host).expect("Invalid node");
        let c_node: *const libc::c_char = node.as_ptr() as *const libc::c_char;
        let port = CString::new(service).expect("Invalid port");
        let c_port: *const libc::c_char = port.as_ptr() as *const libc::c_char;

        println!("Starting server in {host}:{service}");
        libc::getaddrinfo(c_node, c_port, &hints, &mut servinfo)
    };
    if rv != 0 {
        eprintln!("getaddrinfo: {}", unsafe {
            CStr::from_ptr(libc::gai_strerror(rv)).to_str().unwrap()
        });
        return;
    }
    todo!()
}
