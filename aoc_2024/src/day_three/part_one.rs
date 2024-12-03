use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let regex = Regex::new(r"mul\(([0-9]{1,3})+,([0-9]{1,3})+\)")?;
    let output = regex
        .captures_iter(
            buffer
                .lines()
                .map_while(Result::ok)
                .collect::<String>()
                .as_str(),
        )
        .map(|item| {
            item[1].parse::<usize>().expect("Wasn't a number")
                * item[2].parse::<usize>().expect("wasn't a number")
        })
        .sum();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn day_three_part_one() -> Result<(), Box<dyn Error>> {
        let output = part_one("./assets/day_three/sample.txt")?;
        assert_eq!(161, output);
        Ok(())
    }
}
