use core::f32;

fn hton(number: f32) -> u32 {
    let mut number = number;
    let sign = if number < 0.0 {
        number = -number;
        1u32
    } else {
        0u32
    };
    //   p = ((((uint32_t)f) & 0x7fff) << 16) | (sign << 31);
    let mut p = ((number.to_bits() & 0x7fff) << 16u32) | (sign << 31);
    p |= ((number - number.trunc()) * 65536.0f32).to_bits() & 0xffff;
    p
}

pub fn marshall() {
    let value = f32::consts::PI;
    let converted_u32 = hton(value);
    println!("Original {}", value);
    println!("Network 0x{:08X}", converted_u32);
}
