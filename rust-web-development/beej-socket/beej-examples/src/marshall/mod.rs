use core::f32;

fn hton(number: f32) -> u32 {
    let mut number = number;
    let sign = if number < 0.0 {
        number = -number;
        1u32
    } else {
        0u32
    };
    //    p = ((((uint32_t)f) & 0x7fff) << 16) | (sign << 31);
    let mut p = (((number as u32) & 0x7fff) << 16u32) | (sign << 31);
    p |= (((number - number.trunc()) * 65536.0f32) as u32) & 0xffff;
    p
}

fn ntoh(number: u32) -> f32 {
    // float f = ((p >> 16) & 0x7fff);
    // f += (p & 0xffff) / 65536.0f;

    // if (((p >> 31) & 0x1) == 0x1) {
    //   f = -1;
    // }
    // return f;
    let mut float: f32 = ((number >> 16) & 0x7fff) as f32;
    float += (number & 0xffff) as f32 / 65536.0f32;
    todo!()
}

pub fn marshall() {
    let value = f32::consts::PI;
    let converted_u32 = hton(value);
    println!("Original {}", value);
    println!("Network 0x{:08X}", converted_u32);
}
