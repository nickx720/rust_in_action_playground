use std::fmt::format;

use anyhow::{Context, bail};

pub fn day6_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = input
        .trim()
        .split("\n")
        .map(|item| item.trim().split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let rows = input.len();
    let col = input[0].len();
    // 0 1 2 3
    // 1 2 3
    let mut total = 0usize;
    for index in 0..col {
        let mut op = ' ';
        let mut nums = vec![];
        for second_index in (0..rows).rev() {
            if input[second_index][index] == "*" {
                op = '*';
                continue;
            }
            if input[second_index][index] == "+" {
                op = '+';
                continue;
            }
            let number = input[second_index][index]
                .parse::<usize>()
                .with_context(|| format!("Parsing failed for {}{}", second_index, index))?;
            nums.push(number);
        }
        match op {
            '+' => {
                total += nums
                    .iter()
                    .skip(1)
                    .fold(nums[0], |acc, &x| acc.saturating_add(x))
            }
            '*' => {
                total += nums
                    .iter()
                    .skip(1)
                    .fold(nums[0], |acc, &x| acc.saturating_mul(x))
            }
            _ => bail!("Whoops invalid op"),
        }
    }
    Ok(total)
}

// TODO(day6 part2)
// 1) dont split_whitespace, keep spaces (treat input like a grid)
// 2) pad each line to same width (right pad w/ spaces)
// 3) scan columns right -> left
// 4) for each column: read top->bottom (except op row) and glue digits into a string
// 5) if digit string not empty -> parse -> push into operands
// 6) if op row at this col is + or * -> fold operands, add to total, clear operands
// 7) (debug) print vertical slices like "623+" as you scan
// [src/day6/mod.rs:83:13] input[second_index][index] as char = ' '
pub fn day6_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let mut input = input
        .trim()
        .split("\n")
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let width = input
        .iter()
        .map(|item| item.len())
        .max()
        .with_context(|| format!("Couldn't parse"))?;
    input.iter_mut().for_each(|item| {
        let pad = width.saturating_sub(item.len());
        item.push_str(&" ".repeat(pad));
    });
    let input = input
        .iter()
        .map(|item| item.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let rows = input.len();
    let mut total = 0usize;

    for index in (0..width).rev() {
        let mut nums = vec![];
        for second_index in 0..rows {
            let item = input[second_index][index];
            match item {
                b'+' => {
                    total += nums
                        .iter()
                        .skip(1)
                        .fold(nums[0], |acc: usize, &x| acc.saturating_add(x))
                }
                b'*' => {
                    total += nums
                        .iter()
                        .skip(1)
                        .fold(nums[0], |acc, &x| acc.saturating_mul(x))
                }
                b'0'..=b'9' => {
                    let n = (item - b'0') as usize;
                    nums.push(n);
                }
                _ => continue,
            }
        }
    }

    Ok(total)
}
