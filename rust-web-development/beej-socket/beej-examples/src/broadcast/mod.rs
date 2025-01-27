use std::net::{IpAddr, SocketAddrV4, SocketAddrV6};

pub fn socketbroadcaster(host: IpAddr, port: u16, message: String) {
    match host {
        IpAddr::V4(addr) => {
            let socket = SocketAddrV4::new(addr, port);
        }
        IpAddr::V6(addr) => {
            let socket = SocketAddrV6::new(addr, port, 0, 0);
        }
    }
    todo!()
}
