use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn part_two(path: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))")?;
    let operand = Regex::new(r"\d{1,3}")?;
    let mut output = 0;
    let mut multiply = true;
    for item in regex.find_iter(
        buffer
            .lines()
            .map_while(Result::ok)
            .collect::<String>()
            .as_str(),
    ) {
        let pattern = item.as_str();
        match pattern {
            "do()" => multiply = true,
            "don't()" => multiply = false,
            _ => {
                if !multiply {
                    continue;
                }
                let mut num = operand.find_iter(pattern);
                let (a, b) = (
                    num.next()
                        .expect("Nothing found")
                        .as_str()
                        .parse::<i32>()
                        .expect("Couldn;t parse"),
                    num.next()
                        .expect("Nothing found")
                        .as_str()
                        .parse::<i32>()
                        .expect("Couldn't parse"),
                );
                output += a * b;
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn day_three_part_two() -> Result<(), Box<dyn Error>> {
        let output = part_two("./assets/day_three/sample2.txt")?;
        assert_eq!(48, output);
        Ok(())
    }
}
