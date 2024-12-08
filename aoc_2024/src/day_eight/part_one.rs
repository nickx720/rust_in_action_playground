use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let input = buf.lines().map_while(Result::ok).collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();
    let mut grid = input
        .iter()
        .flat_map(|item| item.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    let mut pos = [const { Vec::new() }; 128];
    for pos_y in 0..height {
        for pos_x in 0..width {
            let chracter = grid[pos_y * width + pos_x];
            if chracter != b'.' {
                pos[chracter as usize].push((pos_x as i32, pos_y as i32));
            }
        }
    }
    let mut output = 0;
    for antennas in pos {
        for item in 0..antennas.len() {
            for second_item in 0..antennas.len() {
                if item == second_item {
                    continue;
                }

                let pos_x = antennas[second_item].0 - antennas[item].0;
                let pos_y = antennas[second_item].1 - antennas[item].1;
                for number in 1..i32::MAX {
                    let current_x = antennas[item].0 + pos_x * number;
                    let current_y = antennas[item].1 + pos_y * number;

                    if current_x >= 0
                        && current_y >= 0
                        && current_x < width as i32
                        && current_y < height as i32
                    {
                        let number_index = current_y as usize * width + current_x as usize;
                        if number == 2 && grid[number_index] != 1 {
                            output += 1;
                            grid[number_index] = 1;
                        }
                    } else {
                        break;
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
    fn day_eight_part_one() -> Result<(), Box<dyn Error>> {
        let output = part_one("./assets/day_eight/sample.txt")?;
        assert_eq!(14, output);
        Ok(())
    }
}
