use typed_builder::TypedBuilder;
use types::{Family, Flag, SocketType};

#[derive(PartialEq, TypedBuilder)]
pub struct AddrInfo {
    #[builder(default=Family::Unspecified)]
    family: Family,
    #[builder(default=SocketType::Stream)]
    socktype: SocketType,
    #[builder(default=Flag::None)]
    flags: Flag,
}

impl From<libc::addrinfo> for AddrInfo {
    fn from(value: libc::addrinfo) -> Self {
        Self {
            family: value.ai_family.into(),
            socktype: value.ai_socktype.into(),
            flags: value.ai_flags.into(),
        }
    }
}
