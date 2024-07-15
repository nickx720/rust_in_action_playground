use builders::AddrInfo;
use nix::libc::{self};
use socket2::SockAddr;
use std::{error::Error, ffi::CString, ptr};

use types::Family;

pub fn show_ip(host: String, family: Family, service: String) -> Result<(), Box<dyn Error>> {
    dbg!(&host, &family, &service);
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
        res = (unsafe { *res }).ai_next as *mut libc::addrinfo;
        println!("\t{}", sockaddr.as_socket().expect("Failed to extract IP"));
        match sockaddr.family() as i32 {
            libc::AF_INET => {
                println!("\t Family: IPV4");
            }
            libc::AF_INET6 => {
                println!("\t Family: IPV6");
            }
            _ => {
                println!("\t Unknown family");
            }
        }
    }
    Ok(())
}
