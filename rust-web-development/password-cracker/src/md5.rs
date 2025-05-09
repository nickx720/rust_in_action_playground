pub fn md5(input: String) {
    let length = input.len().to_le_bytes();
    let mut message = input.clone().to_string().into_bytes();
    let s: Vec<u32> = vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let mut k: Vec<u32> = vec![0; 64];
    // https://en.wikipedia.org/wiki/MD5?utm_source=substack&utm_medium=email#Algorithm
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
    k[36] = 0xa4beea44;
    k[37] = 0x4bdecfa9;
    k[38] = 0xf6bb4b60;
    k[39] = 0xbebfbc70;
    k[40] = 0x289b7ec6;
    k[41] = 0xeaa127fa;
    k[42] = 0xd4ef3085;
    k[43] = 0x04881d05;
    k[44] = 0xd9d4d039;
    k[45] = 0xe6db99e5;
    k[46] = 0x1fa27cf8;
    k[47] = 0xc4ac5665;
    k[48] = 0xf4292244;
    k[49] = 0x432aff97;
    k[50] = 0xab9423a7;
    k[51] = 0xfc93a039;
    k[52] = 0x655b59c3;
    k[53] = 0x8f0ccc92;
    k[54] = 0xffeff47d;
    k[55] = 0x85845dd1;
    k[56] = 0x6fa87e4f;
    k[57] = 0xfe2ce6e0;
    k[58] = 0xa3014314;
    k[59] = 0x4e0811a1;
    k[60] = 0xf7537e82;
    k[61] = 0xbd3af235;
    k[62] = 0x2ad7d2bb;
    k[63] = 0xeb86d391;
    let (mut a0, mut b0, mut c0, mut d0) =
        (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
    let append_one = [0x80u8];
    message.extend_from_slice(&append_one);
    loop {
        // the following is done so that the message length is always a multiple of 64 for the
        // above function to work
        if message.len() % 64 == 56 {
            break;
        }
        message.extend_from_slice(&[0x00]);
    }
    // Appending length after converting it into bytes or length in bits mod 2.pow(64)
    message.extend_from_slice(&length);
    // read in chunk size of 512 bits which is equal to chunk 64 bytes
    for chunk in message.chunks(64) {
        // break the above chunk into 16 different entries, each with a length of 32 bits or 4
        // bytes
        for word in chunk.chunks(4) {
            let mut new_word = [0u32; 16];
            for (i, chunk) in word.chunks_exact(4).enumerate() {
                new_word[i] = u32::from_le_bytes(chunk.try_into().unwrap());
            }
            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;
            for i in 0..64 {
                let mut f = 0u32;
                let mut g = 0u32;
                if 0 == i && i <= 15 {
                    f = (b & c) | ((!b) & d);
                    g = i;
                } else if 16 == i || i <= 31 {
                    f = (d & b) | ((!d) & c);
                    g = (5 * i + 1) % 16;
                } else if 48 == i || i <= 63 {
                    f = c ^ (b | (!d));
                    g = (7 * i) % 16
                }
                // why is len 4 but index is at 6
                let f = f
                    .wrapping_add(a)
                    .wrapping_add(k[i as usize])
                    .wrapping_add(new_word[g as usize]);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(leftroate(f, s[i as usize]));
            }
            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }
    }
    let mut output: Vec<u8> = Vec::new();
    output.extend_from_slice(&a0.to_le_bytes());
    output.extend_from_slice(&b0.to_le_bytes());
    output.extend_from_slice(&c0.to_le_bytes());
    output.extend_from_slice(&d0.to_le_bytes());
    let digest: String = output.iter().map(|item| format!("{:02x}", item)).collect();
    dbg!(digest);
}
//TODO what is left rotate?
fn leftroate(x: u32, y: u32) -> u32 {
    // ensure y doesn't exceed 32 bits
    let y = y % 32;
    (x << y) | (x >> (32 - y))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(1, 1);
    }
    #[test]
    fn test_left_rotate() {
        let preleft = 0b00000000;
        let afterleft = 0b00000000;
        let after = leftroate(preleft, 3);
        assert_eq!(afterleft, after);
        let preleft = 0b00000000_00000000_00000000_11110000;
        let afterleft = 0b00000000_00000000_00001111_00000000;
        let after = leftroate(preleft, 4);
        println!("expected {:0b}   actual: {:0b}", afterleft, after);
        assert_eq!(afterleft, after);
    }
}
