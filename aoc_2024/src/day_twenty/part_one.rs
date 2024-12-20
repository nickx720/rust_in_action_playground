use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use crate::day_twenty::solve;

pub fn part_one<const MIN_SAVINGS: i32>(path: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file)
        .bytes()
        .map_while(Result::ok)
        .collect::<Vec<u8>>();
    let output = solve::<MIN_SAVINGS, 2>(&input);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twenty_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty/sample.txt";
        let output = part_one::<20>(path)?;
        assert_eq!(output, 5);
        Ok(())
    }
}
