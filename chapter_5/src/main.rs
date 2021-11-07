mod foo;
use foo::understanding_endianess;

fn isolating_sign_bit(){
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 1;
    dbg!(n_bits);
    dbg!(format!("{:#b}",42));
}

fn isolating_the_exponent(){
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    let exponent = (exponent_ as i32) - 12;
    dbg!(exponent);
}

fn main() {
    //    same();
    //    interpret_float_string_as_integer();
 //   bit_patterns_translate_to_a_fixed_number_of_integers()
// understanding_endianess()
//isolating_sign_bit()
isolating_the_exponent()
}
