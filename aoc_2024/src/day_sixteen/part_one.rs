use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::day_sixteen::State;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let input = buffer.lines().map_while(Result::ok).collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();
    let grid = input
        .iter()
        .flat_map(|item| item.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    let mut start = (0, 0);
    'o: for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x as i32, y as i32);
                break 'o;
            }
        }
    }
    let mut queue = BinaryHeap::new();
    let start = State {
        score: 0,
        x: start.0,
        y: start.1,
        dx: 1,
        dy: 0,
        path: Vec::new(),
    };
    queue.push(Reverse(start));
    // mapping of places with lowest scores
    let mut seen = vec![usize::MAX - 1000; width * height];

    // min score to get to end
    let mut min = usize::MAX;

    let mut paths = Vec::new();

    while let Some(Reverse(State {
        score,
        x,
        y,
        dx: prev_dx,
        dy: prev_dy,
        path,
    })) = queue.pop()
    {
        if grid[y as usize * width + x as usize] == b'E' {
            if score > min {
                break;
            }
            paths.push(path.clone());
            min = score;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            let nscore = if (prev_dx == 0 && prev_dy == 0) || dx == prev_dx && dy == prev_dy {
                score + 1
            } else {
                score + 1001
            };

            let last_seen_score = seen[ny as usize * width + nx as usize];

            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] != b'#'
                && !(prev_dx == 0 && prev_dy == -dy)
                && !(prev_dx == -dx && prev_dy == 0)
                && nscore <= last_seen_score + 1000
            {
                seen[ny as usize * width + nx as usize] = nscore;
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                queue.push(Reverse(State {
                    score: nscore,
                    x: nx,
                    y: ny,
                    dx,
                    dy,
                    path: new_path,
                }));
            }
        }
    }
    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_sixteen_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_sixteen/sample.txt";
        let output = part_one(path)?;
        assert_eq!(7036, output);
        Ok(())
    }
}
