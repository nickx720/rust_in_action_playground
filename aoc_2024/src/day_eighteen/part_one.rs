use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::{explore_at_time, TInput};

pub fn part_one(path: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|item| {
            let (x, y) = item.split_once(",").expect("In valid input format");

            (
                x.parse().expect("Parsing failed"),
                y.parse().expect("Parsing failed"),
            )
        })
        .collect::<TInput>();
    let to_simulate = if input.len() > 1000 { 1024 } else { 12 };
    let output = explore_at_time(&input, to_simulate).expect("Invalid input");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_eighteen_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_eighteen/sample.txt";
        let output = part_one(path)?;
        assert_eq!(22, output);
        Ok(())
    }
}
