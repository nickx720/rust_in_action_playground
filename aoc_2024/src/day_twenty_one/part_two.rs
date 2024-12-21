use std::{collections::HashMap, error::Error, fs::File, io::Read};

use crate::day_twenty_one::find_shortest_sequence;

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let numeric = vec![
        [b'7', b'8', b'9'],
        [b'4', b'5', b'6'],
        [b'1', b'2', b'3'],
        [b' ', b'0', b'A'],
    ];

    let diagonal = vec![[b' ', b'^', b'A'], [b'<', b'v', b'>']];

    let mut cache = HashMap::new();
    let lines = input.lines().collect::<Vec<_>>();
    let max_depth = 25;

    let mut total = 0;
    for l in &lines {
        let mut cursors = vec![b'A'; max_depth + 1];
        let len = find_shortest_sequence(
            l.as_bytes(),
            max_depth,
            true,
            &mut cursors,
            &numeric,
            &diagonal,
            &mut cache,
        );

        let n = l[0..3].parse::<usize>().unwrap();
        total += n * len;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twenty_two_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_two/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, 126384);
        Ok(())
    }
}
