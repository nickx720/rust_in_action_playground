use std::{collections::HashSet, error::Error, fs::File, io::Read};

use crate::day_nineteen::count_ways;

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let mut sections = input.split("\n\n");
    let towel_patterns = sections
        .next()
        .expect("Parsing failed")
        .split(",")
        .map(|item| item.trim().to_string())
        .collect::<HashSet<String>>();
    let designs: Vec<String> = sections
        .next()
        .expect("Parsing failed")
        .lines()
        .map(|item| item.trim().to_string())
        .collect();
    Ok(designs
        .iter()
        .map(|design| count_ways(&towel_patterns, design))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_nineteen_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_nineteen/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, 16);
        Ok(())
    }
}
