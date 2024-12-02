use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

pub fn part_one(path: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let output = reader
        .lines()
        .map_while(Result::ok)
        .map(|item| {
            item.split(" ")
                .map(|item| item.parse::<u32>())
                .map_while(Result::ok)
                .collect::<Vec<u32>>()
                .as_slice()
                .windows(2)
                .map(|pair| (pair[0].abs_diff(pair[1])))
                .collect()
        })
        .filter(|item: &Vec<u32>| item.iter().all(|x| *x >= 1 || *x <= 3))
        .collect::<Vec<Vec<u32>>>();
    dbg!(output);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_two_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_two/sample.txt";
        let output = part_one(path)?;
        assert_eq!(2, output);
        Ok(())
    }
}
