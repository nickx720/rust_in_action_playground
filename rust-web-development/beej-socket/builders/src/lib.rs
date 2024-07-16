use std::mem;

use typed_builder::TypedBuilder;
use types::{Family, Flag, SocketType};

#[derive(PartialEq, TypedBuilder, Debug)]
pub struct AddrInfo {
    #[builder(default=Family::Unspecified)]
    family: Family,
    #[builder(default=SocketType::Stream)]
    socktype: SocketType,
    #[builder(default=Flag::None)]
    flags: Flag,
}

impl From<AddrInfo> for libc::addrinfo {
    fn from(value: AddrInfo) -> Self {
        unsafe {
            let mut addrinfo: libc::addrinfo = mem::zeroed();
            addrinfo.ai_family = value.family.into();
            addrinfo.ai_socktype = value.socktype.into();
            addrinfo.ai_flags = value.flags.into();
            addrinfo
        }
    }
}
//impl Into<libc::addrinfo> for AddrInfo {
//    fn into(self) -> libc::addrinfo {
//        unsafe {
//            let mut addrinfo: libc::addrinfo = mem::zeroed();
//            addrinfo.ai_family = self.family.into();
//            addrinfo.ai_socktype = self.socktype.into();
//            addrinfo.ai_flags = self.flags.into();
//            addrinfo
//        }
//    }
//}
