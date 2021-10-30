#[allow(dead_code)]
fn same(){
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("a : {:016b} {}",a,a);
    println!("b : {:016b} {}",b,b);
}

fn interpret_float_string_as_integer(){
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

fn bit_patterns_translate_to_a_fixed_number_of_integers(){
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;

    let sixtyfivethousand_533:u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534:u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535:u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}", zero, one , two);
    println!("{}, {}, {}", sixtyfivethousand_533,sixtyfivethousand_534, sixtyfivethousand_535);
}

fn main() {
    //    same();
    //    interpret_float_string_as_integer();
    bit_patterns_translate_to_a_fixed_number_of_integers()
}
