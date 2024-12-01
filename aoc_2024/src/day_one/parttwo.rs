use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

pub fn part_two(file: &str) -> Result<u32, Box<dyn Error>> {
    let mut first_set = Vec::new();
    let mut second_hash: HashMap<u32, u32> = HashMap::new();
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut items = line.split(" ");
        if let (Some(first), Some(second)) = (items.next(), items.last()) {
            let first = first.parse::<u32>()?;
            let second = second.parse::<u32>()?;
            first_set.push(first);
            if second_hash.contains_key(&second) {
                let count = second_hash.get(&second).unwrap();
                second_hash.insert(second, count + 1);
            } else {
                second_hash.insert(second, 1);
            }
        }
    }
    let similarity: u32 = first_set
        .iter()
        .map(|item| {
            if second_hash.contains_key(item) {
                let count = second_hash.get(item).unwrap();
                item * count
            } else {
                0
            }
        })
        .sum();
    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let output = part_two("./assets/day_one/sample.txt").unwrap();
        assert_eq!(output, 31);
    }
}
