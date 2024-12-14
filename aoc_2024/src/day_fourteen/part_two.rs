use std::{error::Error, fs::File, io::Read};

use super::{HEIGHT, WIDTH};

pub fn part_two(path: &str) -> Result<u16, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let mut robots: Vec<_> = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split(&['p', 'v', '=', ',', ' '])
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap());

            let px = nums.next().unwrap();
            let py = nums.next().unwrap();
            let vx = nums.next().unwrap();
            let vy = nums.next().unwrap();
            [px, py, vx, vy]
        })
        .collect();

    let mut tiles = [[0u16; HEIGHT as usize]; WIDTH as usize];
    let mut output = 0;

    loop {
        output += 1;

        for robot in &mut robots {
            robot[0] = (robot[0] + robot[2]).rem_euclid(WIDTH);
            robot[1] = (robot[1] + robot[3]).rem_euclid(HEIGHT);

            tiles[robot[0] as usize][robot[1] as usize] = output;
        }

        let adjacent_robots = robots
            .iter()
            .filter(|&&[x, y, _, _]| {
                [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]]
                    .into_iter()
                    .any(|[x, y]| {
                        (0..WIDTH).contains(&x)
                            && x < WIDTH
                            && (0..HEIGHT).contains(&y)
                            && y < HEIGHT
                            && tiles[x as usize][y as usize] == output
                    })
            })
            .count();

        if adjacent_robots > robots.len() / 2 {
            break Ok(output);
        }
    }
}
