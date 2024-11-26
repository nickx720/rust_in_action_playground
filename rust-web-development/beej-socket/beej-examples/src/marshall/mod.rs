use core::f32;

fn hton(number: f32) -> u32 {
    let sign = if number > 0.0 { 1u32 } else { 0u32 };
    //   p = ((((uint32_t)f) & 0x7fff) << 16) | (sign << 31);
<<<<<<< HEAD
    let p = number.round() & 0x7fff;
=======
    let p = ((number.to_bits() & 0x7fff) << 16u32) | (sign << 31);
>>>>>>> nov26-fixed-conversion
    println!("{}", &p);
    todo!()
}

pub fn marshall() {
    let value = f32::consts::PI;
    let converted_u32 = hton(value);
}
