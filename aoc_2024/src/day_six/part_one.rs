use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use super::turn;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut buffer = BufReader::new(file);
    let mut input = String::new();
    let _ = buffer.read_to_string(&mut input);
    let mut avoid = HashSet::new();
    let mut guard_pos = (0, 0);
    let bottom_right = (
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
    );
    input.lines().enumerate().for_each(|(pos_y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(pos_x, character)| match character {
                '#' => {
                    avoid.insert((pos_x as i32, pos_y as i32));
                }
                '^' => guard_pos = (pos_x as i32, pos_y as i32),
                _ => {}
            })
    });
    let mut visited = HashSet::from([guard_pos]);
    let mut direction = (0, -1);
    let mut next_position = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
    while next_position.0 >= 0
        && next_position.0 < bottom_right.0
        && next_position.1 >= 0
        && next_position.1 < bottom_right.1
    {
        if avoid.contains(&next_position) {
            direction = turn(direction);
            next_position = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        } else {
            guard_pos = next_position;
            visited.insert(guard_pos);
            next_position = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        }
    }
    Ok(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_six_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_six/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 41);
        Ok(())
    }
}
