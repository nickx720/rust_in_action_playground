use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn traverse(input: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    if !(-1..=1)
        .flat_map(|posx| (-1..=1).map(move |posy| (posx, posy)))
        .filter(|(x, y)| *x != 0 && *y != 0)
        .all(|(x, y)| {
            let x = x + pos.0;
            let y = y + pos.1;
            x >= 0 && x < input[0].len() as i32 && y >= 0 && y < input.len() as i32
        })
    {
        return false;
    }
    let chars = [
        input[(pos.1 + 1) as usize][(pos.0 + 1) as usize],
        input[(pos.1 - 1) as usize][(pos.0 - 1) as usize],
        input[(pos.1 - 1) as usize][(pos.0 + 1) as usize],
        input[(pos.1 + 1) as usize][(pos.0 - 1) as usize],
    ];

    chars.iter().filter(|&&c| c == 'S').count() == 2
        && chars.iter().filter(|&&c| c == 'M').count() == 2
        && chars[0] != chars[1]
}
pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
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
            if input[index][second_index] == 'A'
                && traverse(&input, (second_index as i32, index as i32))
            {
                output += 1;
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_four_part_two() -> Result<(), Box<dyn Error>> {
        let output = part_two("./assets/day_four/sample.txt")?;
        assert_eq!(9, output);
        Ok(())
    }
}
