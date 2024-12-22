use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::day_twenty_two::next;

pub fn part_one(path: &str) -> Result<i64, Box<dyn Error>> {
    let file = File::open(path)?;
    let output = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|item| {
            let mut item = item.trim().parse().expect("Not a number");
            for _ in 1..=2000 {
                next(&mut item);
            }
            item
        })
        .sum();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn day_twenty_two_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_two/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 37327623);
        Ok(())
    }
}
