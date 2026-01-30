pub struct TarHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    checksum: [u8; 8],
    linkflag: [u8; 1],
    linkname: [u8; 100],
    pad: [u8; 255], //                   char name[100];
                    //                   char mode[8];
                    //                   char uid[8];
                    //                   char gid[8];
                    //                   char size[12];
                    //                   char mtime[12];
                    //                   char checksum[8];
                    //                   char linkflag[1];
                    //                   char linkname[100];
                    //                   char pad[255];
}

impl TarHeader {
    pub fn new() -> Self {
        Self {
            name: [0u8; 100],
            mode: [0u8; 8],
            uid: [0u8; 8],
            gid: [0u8; 8],
            size: [0u8; 12],
            mtime: [0u8; 12],
            checksum: [0u8; 8],
            linkflag: [0u8; 1],
            linkname: [0u8; 100],
            pad: [0u8; 255],
        }
    }
}
