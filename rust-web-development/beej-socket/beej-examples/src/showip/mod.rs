use builders::AddrInfo;
use nix::libc;
use socket2::SockAddr;
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
    let hints: libc::addrinfo = addrinfo.into();
    let mut res = ptr::null_mut();
    unsafe { libc::getaddrinfo(c_host, service, &hints, &mut res) };
    while !res.is_null() {
        let ((), sockaddr) = unsafe {
            SockAddr::try_init(|storage, len| {
                *len = (*res).ai_addr as _;
                std::ptr::copy_nonoverlapping(
                    (*res).ai_addr as *const u8,
                    storage as *mut u8,
                    (*res).ai_addrlen as usize,
                );
                Ok(())
            })
        }
        .expect("to create a socket");
    }
    todo!()
}
