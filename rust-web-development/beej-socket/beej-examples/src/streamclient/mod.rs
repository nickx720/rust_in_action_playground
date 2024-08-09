use std::{ffi::CString, mem, net::IpAddr, ptr};

use builders::AddrInfo;

fn showaddrinfo(addr: &libc::addrinfo) {
    let ip = unsafe {
        match (*addr).ai_family {
            libc::AF_INET => {
                let addr = (*addr).ai_addr as *const libc::sockaddr_in;
                let addr = &*addr;
                let ip = addr.sin_addr;
                let ip = ip.s_addr;
                let ip = ip.to_be();
                let ip = IpAddr::V4(ip.into());
                ip
            }
            libc::AF_INET6 => {
                let addr = (*addr).ai_addr as *const libc::sockaddr_in6;
                let addr = &*addr;
                let ip = addr.sin6_addr;
                let ip = ip.s6_addr;
                let ip = IpAddr::V6(ip.into());
                ip
            }
            _ => {
                panic!("Unknown family");
            }
        }
    };
    println!("IP: {:?}", ip);
}

pub fn streamclient(host: String) {
    let service = "3490";

    let hints = AddrInfo::builder()
        .family(types::Family::Unspecified)
        .socktype(types::SocketType::Stream)
        .build();
    let mut servinfo = ptr::null_mut();
    unsafe {
        let node = CString::new(host).expect("Invalid host");
        let c_node: *const libc::c_char = node.as_ptr() as *const libc::c_char;
        let service = CString::new(service).expect("Invalid service");
        let c_service: *const libc::c_char = service.as_ptr() as *const libc::c_char;
        let hints = hints.into();
        libc::getaddrinfo(c_node, c_service, &hints, &mut servinfo);
    }
    let mut sockfd = mem::MaybeUninit::<libc::c_int>::uninit();
    while !servinfo.is_null() {
        unsafe {
            let _sockfd = libc::socket(
                (*servinfo).ai_family,
                (*servinfo).ai_socktype,
                (*servinfo).ai_protocol,
            );
            if _sockfd == -1 {
                servinfo = (*servinfo).ai_next as *mut libc::addrinfo;
                continue;
            }
            let errr = libc::connect(_sockfd, (*servinfo).ai_addr, (*servinfo).ai_addrlen);
            if errr == -1 {
                libc::close(_sockfd);
                servinfo = (*servinfo).ai_next as *mut libc::addrinfo;
                eprintln!("client: connect err");
                continue;
            }
            sockfd.write(_sockfd)
        }
        break;
    }
    // if servinfo.is_null()
    todo!()
}
