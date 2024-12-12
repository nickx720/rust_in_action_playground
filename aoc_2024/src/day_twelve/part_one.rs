use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::{Grid, Position};

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let input = input
        .lines()
        .map_while(Result::ok)
        .map(|item| item.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut output = 0;
    let mut visited = HashSet::new();
    let grid = Grid(input);
    for y in 0..grid.ylen() {
        for x in 0..grid.xlen() {
            let position = Position(x as i32, y as i32);
            if visited.contains(&position) {
                continue;
            }
            let mut perimter = 0;
            let mut queue = Vec::new();
            queue.push(position);
            let mut second_visited = HashSet::new();
            while let Some(pos) = queue.pop() {
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);
                second_visited.insert(pos);
                perimter += grid.outer_count(pos);
                for neighbor in grid.neighbors(pos) {
                    queue.push(neighbor)
                }
            }
            output += perimter * second_visited.len()
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twelve_part_one() -> Result<(), Box<dyn Error>> {
        let output = part_one("./assets/day_twelve/sample.txt")?;
        assert_eq!(1930, output);
        Ok(())
    }
}
