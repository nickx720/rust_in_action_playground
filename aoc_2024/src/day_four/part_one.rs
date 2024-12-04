use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const CHRISTMAS_ME: [char; 3] = ['M', 'A', 'S'];

fn traverse(input: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    CHRISTMAS_ME.iter().enumerate().all(|(index, character)| {
        let index = index as i32;
        let pos_x = pos.0 + dir.0 * (1 + index);
        let pos_y = pos.1 + dir.1 * (1 + index);
        pos_x >= 0
            && pos_x < input[0].len() as i32
            && pos_y >= 0
            && pos_y < input.len() as i32
            && input[(pos.1 + dir.1 * (1 + index)) as usize][(pos.0 + dir.0 * (1 + index)) as usize]
                == *character
    })
}

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file);
    let input = lines
        .lines()
        .map_while(Result::ok)
        .map(|item| item.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut output = 0;
    for index in 0..input.len() {
        for second_index in 0..input[0].len() {
            for third_index in -1..=1 {
                for fourth_index in -1..=1 {
                    // ignore current item
                    if third_index == 0 && fourth_index == 0 {
                        continue;
                    }
                    if input[index][second_index] == 'X'
                        && traverse(
                            &input,
                            (second_index as i32, index as i32),
                            (third_index, fourth_index),
                        )
                    {
                        output += 1;
                    }
                }
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn day_four_part_one() -> Result<(), Box<dyn Error>> {
        let output = part_one("./assets/day_four/sample.txt")?;
        assert_eq!(18, output);
        Ok(())
    }
}
