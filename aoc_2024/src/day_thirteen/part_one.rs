use std::{error::Error, fs::File, io::Read};

use super::star_algo_parser;

pub fn part_one(path: &str) -> Result<i64, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    Ok(star_algo_parser(&input, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_thirteen_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_thirteen/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 480);
        Ok(())
    }
}
