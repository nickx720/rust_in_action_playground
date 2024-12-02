use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn validate(val: &Vec<i32>) -> bool {
    let mut sorted_asc = val.clone();
    sorted_asc.sort();

    let mut sorted_desc = sorted_asc.clone();
    sorted_desc.reverse();

    if val != &sorted_asc && val != &sorted_desc {
        return false;
    }

    for i in 1..val.len() {
        if (val[i] - val[i - 1]).abs() > 3 || (val[i] - val[i - 1]).abs() < 1 {
            return false;
        }
    }

    true
}

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let output = reader
        .lines()
        .map_while(Result::ok)
        .map(|item| {
            item.split_whitespace()
                .map(|item| item.parse::<i32>())
                .map_while(Result::ok)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Ok(output.iter().filter(|item| validate(item)).count())
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
