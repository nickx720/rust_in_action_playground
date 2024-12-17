use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::day_seventeen::run;

pub fn part_two(path: &str) -> Result<u64, Box<dyn Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let input = buffer
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let a = input[0][12..].parse::<u64>().expect("Conversion failed");
    let b = input[1][12..].parse::<u64>().expect("Conversion failed");
    let c = input[2][12..].parse::<u64>().expect("Conversion failed");

    let program = input[4][9..]
        .split(",")
        .map(|item| item.parse::<u64>().expect("Parsing failed"))
        .collect::<Vec<_>>();
    let mut factors = vec![0; program.len()];
    loop {
        let mut initialize_a = 0;
        for (index, factor) in factors.iter().enumerate() {
            initialize_a += 8u64.pow(index as u32) * factor;
        }

        let output = run(initialize_a, b, c, &program);
        if output == program {
            break Ok(initialize_a);
        }
        for item in (0..program.len()).rev() {
            if output.len() < item {
                factors[item] += 1;
                break;
            }
            if output[item] != program[item] {
                factors[item] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_seventeen_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_seventeen/sample_two.txt";
        let output = part_two(path)?;
        assert_eq!(output, 117440);
        Ok(())
    }
}
