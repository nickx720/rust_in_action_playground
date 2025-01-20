use core::f32;

fn hton(number: f32) -> u32 {
    let mut number = number;
    let sign = if number < 0.0 {
        number = -number;
        1u32
    } else {
        0u32
    };
    let mut p = (((number as u32) & 0x7fff) << 16u32) | (sign << 31);
    p |= (((number - number.trunc()) * 65536.0f32) as u32) & 0xffff;
    p
}

fn ntoh(number: u32) -> f32 {
    let mut float: f32 = ((number >> 16) & 0x7fff) as f32;
    float += (number & 0xffff) as f32 / 65536.0f32;
    if ((number >> 31) & 0x1) == 0x1 {
        float = -1f32;
    }
    float
}

pub fn marshall() {
    let value = f32::consts::PI;
    let converted_u32 = hton(value);
    println!("Original {}", value);
    println!("Network 0x{:08X}", converted_u32);
    let converted_val = ntoh(converted_u32);
    println!("Reformatted {}", converted_val);
}
