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
    if rows == 0 {
        return Ok(0);
    }
    let op_row = rows - 1;
    let mut total = 0usize;

    let mut operands: Vec<usize> = vec![];
    for index in (0..width).rev() {
        let mut digits: Vec<usize> = vec![];
        for row in 0..op_row {
            let item = input[row][index];
            if (b'0'..=b'9').contains(&item) {
                digits.push((item - b'0') as usize);
            }
        }

        if !digits.is_empty() {
            let combined = digits.iter().fold(0, |acc, &num| acc * 10 + num);
            operands.push(combined);
        }

        match input[op_row][index] {
            b'+' => {
                if !operands.is_empty() {
                    total += operands
                        .iter()
                        .skip(1)
                        .fold(operands[0], |acc, &x| acc.saturating_add(x));
                }
                operands.clear();
            }
            b'*' => {
                if !operands.is_empty() {
                    total += operands
                        .iter()
                        .skip(1)
                        .fold(operands[0], |acc, &x| acc.saturating_mul(x));
                }
                operands.clear();
            }
            _ => {}
        }
    }

    Ok(total)
}
