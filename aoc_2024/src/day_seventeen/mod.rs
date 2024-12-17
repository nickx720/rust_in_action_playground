pub mod part_one;
pub mod part_two;

pub fn run(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut output = Vec::new();
    while ip < program.len() {
        let opcode = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!(),
        };
        match opcode {
            0 => a /= 2u64.pow(combo as u32),
            1 => b ^= literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a / 2u64.pow(combo as u32),
            7 => c = a / 2u64.pow(combo as u32),
            _ => panic!(),
        };
        ip += 2;
    }
    output
}
