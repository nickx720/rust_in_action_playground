use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::{explore_at_time, TInput};

pub fn part_two(path: &str) -> Result<String, Box<dyn Error>> {
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
    let mut possible = 0;
    let mut impossible = input.len() - 1;
    while impossible > (possible + 1) {
        let midpoint = (possible + impossible) / 2;
        if explore_at_time(&input, midpoint).is_some() {
            possible = midpoint;
        } else {
            impossible = midpoint;
        }
    }
    Ok(format!("{},{}", input[possible].0, input[possible].1))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_eighteen_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_eighteen/sample.txt";
        let output = part_two(path)?;
        assert_eq!("6,1", output);
        Ok(())
    }
}
