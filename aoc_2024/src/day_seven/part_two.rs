use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::validate;

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let mut output = 0;
    for item in input.lines() {
        let inner_item = item?;
        let (res, num) = inner_item.split_once(": ").expect("Splitting error");
        let res = res.parse::<usize>()?;
        let num = num
            .split_whitespace()
            .map(|item| item.parse::<usize>().expect("Parsing failed"))
            .collect::<Vec<usize>>();
        if validate(res, &num, num.len() - 1, true) {
            output += res;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_seven_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_seven/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, 11387);
        Ok(())
    }
}
