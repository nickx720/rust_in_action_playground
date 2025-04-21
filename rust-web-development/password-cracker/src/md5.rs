pub fn md5() {
    let s: Vec<u32> = vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let mut k: Vec<u32> = vec![0; 63];
    //https://en.wikipedia.org/wiki/MD5?utm_source=substack&utm_medium=email#Algorithm
    k[0] = 0xd76aa478;
    k[1] = 0xe8c7b756;
    k[2] = 0x242070db;
    k[3] = 0xc1bdceee;
    k[4] = 0xf57c0faf;
    k[5] = 0x4787c62a;
    k[6] = 0xa8304613;
    k[7] = 0xfd469501;
    k[8] = 0x698098d8;
    k[9] = 0x8b44f7af;
    k[10] = 0xffff5bb1;
    k[11] = 0x895cd7be;
    k[12] = 0x6b901122;
    k[13] = 0xfd987193;
    k[14] = 0xa679438e;
    k[15] = 0x49b40821;
    k[16] = 0xf61e2562;
    k[17] = 0xc040b340;
    k[18] = 0x265e5a51;
    k[19] = 0xe9b6c7aa;
    k[20] = 0xd62f105d;
    k[21] = 0x02441453;
    k[22] = 0xd8a1e681;
    k[23] = 0xe7d3fbc8;
    k[24] = 0x21e1cde6;
    k[25] = 0xc33707d6;
    k[26] = 0xf4d50d87;
    k[27] = 0x455a14ed;
    k[28] = 0xa9e3e905;
    k[29] = 0xfcefa3f8;
    k[30] = 0x676f02d9;
    k[31] = 0x8d2a4c8a;
    k[32] = 0xfffa3942;
    k[33] = 0x8771f681;
    k[34] = 0x6d9d6122;
    k[35] = 0xfde5380c;
    //K[36..39] := { 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70 }
    //K[40..43] := { 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05 }
    //K[44..47] := { 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665 }
    //K[48..51] := { 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039 }
    //K[52..55] := { 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1 }
    //K[56..59] := { 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1 }
    //K[60..63] := { 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 }
    dbg!(k[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        md5();
        assert_eq!(1, 1);
    }
}
