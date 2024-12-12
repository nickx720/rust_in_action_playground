use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::{DirectionEnum, Grid, Position};

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
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
            let mut queue = Vec::new();
            queue.push(position);
            let mut scc = HashSet::new();
            while let Some(position) = queue.pop() {
                if visited.contains(&position) {
                    continue;
                }
                visited.insert(position);
                scc.insert(position);
                for neighbour in grid.neighbors(position) {
                    queue.push(neighbour);
                }
            }
            let mut sides = 0;
            let mut second_visited = HashSet::new();
            // update each node in scc
            for &pos in scc.iter() {
                if grid.outer_count(pos) == 0 {
                    continue;
                }
                for dir in [
                    DirectionEnum::North,
                    DirectionEnum::East,
                    DirectionEnum::South,
                    DirectionEnum::West,
                ] {
                    if let Some(neighbour) = grid.get(pos, dir) {
                        if scc.contains(&neighbour) {
                            continue;
                        }
                    }
                    if second_visited.contains(&(pos, dir)) {
                        continue;
                    }
                    sides += 1;
                    let mut queue = Vec::new();
                    queue.push((pos, dir));
                    while let Some((pos, dir)) = queue.pop() {
                        if second_visited.contains(&(pos, dir)) {
                            continue;
                        }

                        second_visited.insert((pos, dir));
                        // iterate preprendiular here so we can check which are not in sec and nodes
                        // that aren't in border
                        for points in dir.perpendicular() {
                            if let Some(neighbour) = grid.get(pos, points) {
                                if second_visited.contains(&(neighbour, dir)) {
                                    continue;
                                }
                                // if not in scc go on
                                if !scc.contains(&neighbour) {
                                    continue;
                                }
                                // border
                                if let Some(node) = grid.get(neighbour, dir) {
                                    if scc.contains(&node) {
                                        continue;
                                    }
                                }
                                queue.push((neighbour, dir))
                            }
                        }
                    }
                }
            }
            output += sides * scc.len();
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twelve_part_two() -> Result<(), Box<dyn Error>> {
        let output = part_two("./assets/day_twelve/sample.txt")?;
        assert_eq!(1206, output);
        Ok(())
    }
}
