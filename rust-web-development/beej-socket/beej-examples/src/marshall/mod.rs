use core::f32;

fn hton(number: f32) -> u32 {
    let sign = if number > 0.0 { 1u32 } else { 0u32 };
    todo!()
}

pub fn marshall() {
    let value = f32::consts::PI;
    let converted_u32 = hton(value);
}
