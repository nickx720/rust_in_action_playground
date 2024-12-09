use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut input = String::new();
    let _ = BufReader::new(file).read_to_string(&mut input);
    let input = input
        .trim()
        .chars()
        .map(|item| item.to_string().parse::<usize>().expect("Invalid no"))
        .collect::<Vec<_>>();
    let mut disk_holder: Vec<Option<usize>> = Vec::with_capacity(input.iter().sum());
    let mut current_pos = 0;
    let mut number = 0;
    let mut it = input.iter();
    while let Some(flen) = it.next() {
        for _ in current_pos..(current_pos + flen) {
            disk_holder.push(Some(number));
        }
        current_pos += flen;
        number += 1;
        if let Some(space) = it.next() {
            for _ in current_pos..(current_pos + space) {
                disk_holder.push(None);
            }
            current_pos += space;
        }
    }
    let mut rear_pos = disk_holder.len() - 1;
    current_pos = 0;
    while current_pos < rear_pos {
        if disk_holder[rear_pos].is_none() {
            rear_pos -= 1;
            continue;
        }
        if disk_holder[current_pos].is_none() {
            if disk_holder[rear_pos].is_some() {
                disk_holder[current_pos] = disk_holder[rear_pos];
                disk_holder[rear_pos] = None;
            }
            rear_pos -= 1;
        }
        current_pos += 1;
    }
    Ok(disk_holder
        .iter()
        .enumerate()
        .map(|(i, f)| if f.is_none() { 0 } else { i * f.unwrap() })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_nine_part_one() -> Result<(), Box<dyn Error>> {
        let output = part_one("./assets/day_nine/sample.txt")?;
        assert_eq!(output, 1928);
        Ok(())
    }
}
