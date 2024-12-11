use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::blink_counter;

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let input: Vec<usize> = input
        .lines()
        .map_while(Result::ok)
        .flat_map(|item| {
            item.split_whitespace()
                .map(|item| item.parse::<usize>().expect("Number is not valid"))
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut output = 0;
    let mut seen = HashMap::new();
    for item in input {
        output += blink_counter(item, 75, &mut seen);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_eleven_part_two() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
