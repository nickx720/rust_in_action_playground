use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use super::turn;

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
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
    let mut direction = (0, -1);
    let mut visited = HashSet::from([(guard_pos, direction)]);
    let mut clear_sec = HashSet::from([guard_pos]);

    let mut obstacle: Option<(i32, i32)> = None;
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
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
            if obstacle.is_none() && !clear_sec.contains(&next_position) {
                obstacle = Some(next_position);

                let mut new_guard_pos = guard_pos;
                let mut visited_again = HashSet::new();
                let mut new_direction = turn(direction);
                let mut new_next_position = (
                    new_guard_pos.0 + new_direction.0,
                    new_guard_pos.1 + new_direction.1,
                );
                while new_next_position.0 >= 0
                    && new_next_position.0 < bottom_right.0
                    && new_next_position.1 >= 0
                    && new_next_position.1 < bottom_right.1
                {
                    if avoid.contains(&new_next_position) || obstacle == Some(new_next_position) {
                        new_direction = turn(new_direction);
                        new_next_position = (
                            new_guard_pos.0 + new_direction.0,
                            new_guard_pos.1 + new_direction.1,
                        );
                    } else if visited.contains(&(new_next_position, new_direction))
                        || visited_again.contains(&(new_next_position, new_direction))
                    {
                        obstacles.insert(obstacle.unwrap());
                        break;
                    } else {
                        new_guard_pos = new_next_position;
                        visited_again.insert((new_guard_pos, new_direction));
                        new_next_position = (
                            new_guard_pos.0 + new_direction.0,
                            new_guard_pos.1 + new_direction.1,
                        );
                    }
                }
                obstacle = None;
            }
            guard_pos = next_position;
            visited.insert((guard_pos, direction));
            clear_sec.insert(guard_pos);
            next_position = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        }
    }
    Ok(obstacles.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_six_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_six/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, 6);
        Ok(())
    }
}
