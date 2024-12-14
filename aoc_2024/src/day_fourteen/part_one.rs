use std::{error::Error, fs::File, io::Read};

use super::{HALF_HEIGHT, HALF_WIDTH, HEIGHT, WIDTH};

pub fn part_one(path: &str) -> Result<u32, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let mut robots_per_quadrants = [0; 4];

    for line in input.lines() {
        let mut nums = line
            .split(&['p', 'v', '=', ',', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        let px = nums.next().unwrap();
        let py = nums.next().unwrap();
        let vx = nums.next().unwrap();
        let vy = nums.next().unwrap();
        let px = (px + vx * 100).rem_euclid(WIDTH);
        let py = (py + vy * 100).rem_euclid(HEIGHT);

        if let Some(pos) = match (px, py) {
            (HALF_WIDTH, _) => None,
            (_, HALF_HEIGHT) => None,
            (..HALF_WIDTH, ..HALF_HEIGHT) => Some(0),
            (HALF_WIDTH.., ..HALF_HEIGHT) => Some(1),
            (..HALF_WIDTH, HALF_HEIGHT..) => Some(2),
            (HALF_WIDTH.., HALF_HEIGHT..) => Some(3),
        } {
            robots_per_quadrants[pos] += 1;
        }
    }

    Ok(robots_per_quadrants.into_iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_fourteen_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_fourteen/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 21);
        Ok(())
    }
}
