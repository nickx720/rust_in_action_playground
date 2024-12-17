use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::day_seventeen::run;

pub fn part_one(path: &str) -> Result<String, Box<dyn Error>> {
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
    let output = run(a, b, c, &program);
    Ok(output
        .into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_seventeen_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_seventeen/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }
}
