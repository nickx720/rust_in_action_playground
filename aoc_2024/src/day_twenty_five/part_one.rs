use std::{error::Error, fs::read_to_string};

fn height(input: &str) -> Vec<isize> {
    input
        .lines()
        .flat_map(|line| line.chars().enumerate())
        .filter(|(_, c)| *c == '#')
        .fold(vec![-1; 5], |mut heights, (i, _)| {
            heights[i] += 1;
            heights
        })
}

fn fit(key: &[isize], lock: &[isize]) -> bool {
    key.iter().zip(lock.iter()).all(|(&k, l)| k < 6 - l)
}

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let input = read_to_string(path).unwrap();
    let (keys, locks) =
        input
            .split("\n\n")
            .fold((Vec::new(), Vec::new()), |(mut keys, mut locks), group| {
                match group.chars().next() {
                    Some('#') => locks.push(height(group)),
                    _ => keys.push(height(group)),
                }
                (keys, locks)
            });

    let p1 = locks
        .iter()
        .map(|lock| keys.iter().filter(|key| fit(key, lock)).count())
        .sum::<usize>();
    Ok(p1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twenty_five_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_five/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 3);
        Ok(())
    }
}
