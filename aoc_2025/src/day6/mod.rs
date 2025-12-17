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
