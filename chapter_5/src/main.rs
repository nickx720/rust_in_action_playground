mod chip8;
mod foo;
mod mantisa;
mod q7;
use foo::understanding_endianess;
use mantisa::mantisaa_main;

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

fn main() {
    //    same();
    //    interpret_float_string_as_integer();
    //   bit_patterns_translate_to_a_fixed_number_of_integers()
    // understanding_endianess()
    //isolating_sign_bit()
    // mantisaa_main()
    //    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    //    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    //    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
    let mut cpu = chip8::CPU::CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;
    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;
    cpu.run();
    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 = {}", cpu.registers[0]);
}
