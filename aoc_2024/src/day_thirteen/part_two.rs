use std::{error::Error, fs::File, io::Read};

use crate::day_thirteen::star_algo_parser;

pub fn part_two(path: &str) -> Result<i64, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    Ok(star_algo_parser(&input, true))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_thirteen_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_thirteen/sample.txt";
        let output = part_two(path)?;
        assert_ne!(output, 100);
        Ok(())
    }
}
