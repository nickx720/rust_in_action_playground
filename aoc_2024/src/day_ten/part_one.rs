use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::dfs;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let input = buffer
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let width = input[0].len();
    let height = input.len();
    let grid = input
        .iter()
        .map(|item| {
            item.chars()
                .map(|item| item.to_digit(10))
                .map_while(|item| item)
                .map(|item| item as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut trailheads = Vec::new();
    for pos_y in 0..height {
        for pos_x in 0..width {
            if grid[pos_y][pos_x] == 0usize {
                trailheads.push((pos_x, pos_y))
            }
        }
    }
    let mut output = 0;
    for pos_y in 0..height {
        for pos_x in 0..width {
            let mut hash_set = HashSet::new();
            if grid[pos_y][pos_x] == 0usize {
                output += dfs(pos_y, pos_x, &grid, &mut hash_set, true);
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_ten_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_ten/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 36);
        Ok(())
    }
}