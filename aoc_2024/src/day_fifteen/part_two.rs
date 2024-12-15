use std::{cmp::Ordering, error::Error, fs::File, io::Read};

use crate::day_fifteen::{next_coord, next_coords};

use super::{char_to_wide_tiles, get_cell, Direction, WideMap, WideTile};

fn robot_coord_wide(m: &WideMap) -> (usize, usize) {
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, t) in r.iter().enumerate() {
            if matches!(t, WideTile::Robot) {
                return (c_idx, r_idx);
            }
        }
    }
    unreachable!();
}
fn next_empty_tile_coord_wide_horiz(
    mut xy: (usize, usize),
    mve: &Direction,
    map: &WideMap,
) -> Option<(usize, usize)> {
    debug_assert!(matches!(mve, Direction::Left | Direction::Right));
    loop {
        xy = next_coord(xy, mve);
        match get_cell(xy, map) {
            WideTile::BoxLeft => continue,
            WideTile::BoxRight => continue,
            WideTile::Wall => return None,
            WideTile::Empty => return Some(xy),
            WideTile::Robot => unreachable!(),
        }
    }
}
fn shift_boxes_horiz(map: &mut [Vec<WideTile>], start: (usize, usize), end: (usize, usize)) {
    let delta = -(end.0 as isize - start.0 as isize).signum();
    let mut i = end.0 as isize;
    loop {
        if i as usize == start.0 {
            break;
        }
        map[start.1][i as usize] = map[start.1][(i + delta) as usize];
        map[start.1][(i + delta) as usize] = WideTile::Empty;
        i += delta;
    }
}
fn check_moves_vert_wide<const N: usize>(
    coords: [(usize, usize); N],
    mve: &Direction,
    map: &WideMap,
    mut target_moves: Vec<(usize, usize)>,
) -> (bool, Vec<(usize, usize)>) {
    debug_assert!(matches!(mve, Direction::Up | Direction::Down));
    println!("checking moves at coords {:?}", coords);
    let next_coords = next_coords(coords, mve);
    let mut next_can_move = vec![];
    for xy in next_coords {
        let tile = get_cell(xy, map);
        match tile {
            WideTile::Empty => {
                next_can_move.push(true);
                target_moves.push(xy);
            }
            WideTile::BoxLeft => {
                let box_coords = [xy, (xy.0 + 1, xy.1)];
                target_moves.push(xy);
                let a;
                (a, target_moves) = check_moves_vert_wide(box_coords, mve, map, target_moves);
                next_can_move.push(a);
            }
            WideTile::BoxRight => {
                let box_coords = [xy, (xy.0 - 1, xy.1)];
                target_moves.push(xy);
                let a;
                (a, target_moves) = check_moves_vert_wide(box_coords, mve, map, target_moves);
                next_can_move.push(a);
            }
            WideTile::Wall => return (false, target_moves),
            WideTile::Robot => unreachable!(),
        }
    }
    println!("Check moves outcome: {:?}", target_moves);
    (next_can_move.iter().all(|b| *b), target_moves)
}
fn shift_boxes_vert(
    map: &mut [Vec<WideTile>],
    mve: &Direction,
    mut target_moves: Vec<(usize, usize)>,
) {
    println!("Shuffling moves - before dedup {:?}", target_moves);
    target_moves.sort_by(|(x1, y1), (x2, y2)| {
        let order = if matches!(mve, Direction::Down) {
            y1.cmp(y2)
        } else {
            y2.cmp(y1)
        };
        if matches!(order, Ordering::Equal) {
            return x2.cmp(x1);
        }
        order
    });
    target_moves.dedup();
    println!("Shuffling moves - after dedup {:?}", target_moves);
    let rev_delta = match mve {
        Direction::Up => 1,
        Direction::Down => -1,
        Direction::Left | Direction::Right => unreachable!(),
    };
    for mve in target_moves.iter().rev() {
        println!("mve: {:?}", mve);
        map[mve.1][mve.0] = map[mve.1.checked_add_signed(rev_delta).unwrap()][mve.0];
        map[mve.1.checked_add_signed(rev_delta).unwrap()][mve.0] = WideTile::Empty
    }
}
fn apply_move_wide(mut map: Vec<Vec<WideTile>>, mve: Direction) -> Vec<Vec<WideTile>> {
    let robot_pos = robot_coord_wide(&map);
    let maybe_next_pos = next_coord(robot_pos, &mve);
    let next_tile = get_cell(maybe_next_pos, &map);
    match next_tile {
        WideTile::Empty => {
            map[maybe_next_pos.1][maybe_next_pos.0] = WideTile::Robot;
            map[robot_pos.1][robot_pos.0] = WideTile::Empty;
            map
        }
        WideTile::BoxLeft | WideTile::BoxRight
            if matches!(mve, Direction::Left | Direction::Right) =>
        {
            let Some(next_empty) = next_empty_tile_coord_wide_horiz(maybe_next_pos, &mve, &map)
            else {
                return map;
            };
            shift_boxes_horiz(&mut map, robot_pos, next_empty);
            map
        }
        WideTile::BoxLeft => {
            let box_coords = [maybe_next_pos, (maybe_next_pos.0 + 1, maybe_next_pos.1)];
            let (a, moves) = check_moves_vert_wide(box_coords, &mve, &map, vec![]);
            if a {
                shift_boxes_vert(&mut map, &mve, moves);
                map[maybe_next_pos.1][maybe_next_pos.0] = WideTile::Robot;
                map[robot_pos.1][robot_pos.0] = WideTile::Empty;
            }
            map
        }
        WideTile::BoxRight => {
            let box_coords = [maybe_next_pos, (maybe_next_pos.0 - 1, maybe_next_pos.1)];
            let (a, moves) = check_moves_vert_wide(box_coords, &mve, &map, vec![]);
            if a {
                shift_boxes_vert(&mut map, &mve, moves);
                map[maybe_next_pos.1][maybe_next_pos.0] = WideTile::Robot;
                map[robot_pos.1][robot_pos.0] = WideTile::Empty;
            }
            map
        }
        WideTile::Wall => map,
        WideTile::Robot => unreachable!(),
    }
}

fn total_gps_coords_wide(m: &WideMap) -> usize {
    let mut total = 0;
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if matches!(c, WideTile::BoxLeft) {
                total += 100 * r_idx + c_idx;
            }
        }
    }
    total
}
pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| line.chars().flat_map(char_to_wide_tiles).collect())
        .collect::<WideMap>();
    let moves = moves
        .chars()
        .filter(|c| *c != '\n')
        .map(Into::into)
        .collect::<Vec<Direction>>();
    for move_item in moves {
        map = apply_move_wide(map, move_item)
    }
    Ok(total_gps_coords_wide(&map))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_fifteen_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_fifteen/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, 9021);
        Ok(())
    }
}
