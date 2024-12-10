use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

struct FileStat {
    number: usize,
    start: usize,
    len: usize,
}

struct Space {
    start: usize,
    len: usize,
}

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut input = String::new();
    let _ = BufReader::new(file).read_to_string(&mut input);
    let input = input
        .trim()
        .chars()
        .map(|item| item.to_string().parse::<usize>().expect("Invalid no"))
        .collect::<Vec<_>>();
    let mut files: Vec<FileStat> = Vec::new();
    let mut space: Vec<Space> = Vec::new();
    let mut current_pos = 0;
    let mut current_num = 0;
    let mut iterator = input.iter();
    while let Some(item) = iterator.next() {
        files.push(FileStat {
            number: current_num,
            start: current_pos,
            len: *item,
        });
        current_num += 1;
        current_pos += item;
        if let Some(second_item) = iterator.next() {
            space.push(Space {
                start: current_pos,
                len: *second_item,
            });
            current_pos += second_item;
        }
    }
    let mut defrag = files.len() - 1;
    while defrag > 0 {
        for (rs, _) in space.iter().enumerate() {
            if space[rs].len >= files[defrag].len && space[rs].start < files[defrag].start {
                files[defrag].start = space[rs].start;
                space[rs].start += files[defrag].len;
                space[rs].len -= files[defrag].len;
                break;
            }
        }
        defrag -= 1;
    }
    let output = files
        .iter()
        .map(|item| {
            (item.start..(item.start + item.len))
                .map(|inner_item| inner_item * item.number)
                .sum::<usize>()
        })
        .sum::<usize>();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_nine_part_two() -> Result<(), Box<dyn Error>> {
        let output = part_two("./assets/day_nine/sample.txt")?;
        assert_eq!(output, 2858);
        Ok(())
    }
}
