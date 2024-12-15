use std::{error::Error, fs::File, io::Read};

use crate::day_fifteen::Direction;

use super::{get_cell, next_coord, next_empty_tile_coord, robot_coord, Map, Tile};

fn total_gps_coords(m: &Map) -> usize {
    let mut total = 0;
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if matches!(c, Tile::Box) {
                total += 100 * r_idx + c_idx;
            }
        }
    }
    total
}
fn apply_move(mut map: Map, mve: Direction) -> Map {
    let robot_pos = robot_coord(&map);
    let maybe_next_pos = next_coord(robot_pos, &mve);
    let next_tile = get_cell(maybe_next_pos, &map);
    match next_tile {
        Tile::Empty => {
            map[maybe_next_pos.1][maybe_next_pos.0] = Tile::Robot;
            map[robot_pos.1][robot_pos.0] = Tile::Empty;
            map
        }
        Tile::Box => {
            let Some(next_empty) = next_empty_tile_coord(maybe_next_pos, &mve, &map) else {
                return map;
            };
            map[maybe_next_pos.1][maybe_next_pos.0] = Tile::Robot;
            map[robot_pos.1][robot_pos.0] = Tile::Empty;
            map[next_empty.1][next_empty.0] = Tile::Box;
            map
        }
        Tile::Wall => map,
        Tile::Robot => unreachable!(),
    }
}
pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect::<Map>();

    let moves = moves
        .chars()
        .filter(|c| *c != '\n')
        .map(Into::into)
        .collect::<Vec<Direction>>();
    for move_item in moves {
        map = apply_move(map, move_item)
    }
    Ok(total_gps_coords(&map))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_fifteen_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_fifteen/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 10092);
        Ok(())
    }
}
