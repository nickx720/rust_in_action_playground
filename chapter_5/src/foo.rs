pub fn same(){
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("a : {:016b} {}",a,a);
    println!("b : {:016b} {}",b,b);
}

pub fn interpret_float_string_as_integer(){
    let a : f64 = 42.42;
    let frankentype: u64 = unsafe {
        std::mem::transmute(a)
    };
    println!("{} ", frankentype);
    println!("{:032b}", frankentype);

    let b: f64 = unsafe {
        std::mem::transmute(frankentype)
    };
    println!("{}", b);
    assert_eq!(a,b);
}

pub fn bit_patterns_translate_to_a_fixed_number_of_integers(){
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;

    let sixtyfivethousand_533:u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534:u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535:u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}", zero, one , two);
    println!("{}, {}, {}", sixtyfivethousand_533,sixtyfivethousand_534, sixtyfivethousand_535);
}

pub fn understanding_endianess(){
    let big_endian: [u8;4] = [0xAA,0xBB,0xCC,0xDD];
    let little_endian: [u8;4] = [0xDD,0xCC,0xBB,0xAA];
    let a:i32 = unsafe {
        std::mem::transmute(big_endian)
    };
    let b: i32 = unsafe {
        std::mem::transmute(little_endian)
    };
    println!("{} vs {}",a,b);
}
pub fn isolating_sign_bit(){
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 1;
    dbg!(n_bits);
    dbg!(format!("{:#b}",42));
}

pub fn isolating_the_exponent(){
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    let exponent = (exponent_ as i32) - 12;
    dbg!(exponent);
}
