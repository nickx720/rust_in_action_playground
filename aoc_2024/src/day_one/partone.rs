use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

#[allow(dead_code)]
fn part_one(file: &str) -> Result<u32, Box<dyn Error>> {
    let mut first_set = Vec::new();
    let mut second_set = Vec::new();
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut items = line.split(" ");
        if let (Some(first), Some(second)) = (items.next(), items.last()) {
            let first = first.parse::<u32>()?;
            let second = second.parse::<u32>()?;
            first_set.push(first);
            second_set.push(second)
        }
    }
    first_set.sort();
    second_set.sort();
    let distance = first_set
        .iter()
        .zip(second_set)
        .map(|(first, second)| first.abs_diff(second))
        .sum::<u32>();
    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let output = part_one("./assets/day_one/sample.txt").unwrap();
        assert_eq!(output, 11);
    }
}
