use anyhow::{Context, Result};

fn parse_digits(input: &str) -> Result<Vec<usize>, anyhow::Error> {
    input
        .chars()
        .map(|item| {
            item.to_digit(10)
                .with_context(|| format!("Conversion failed"))
                .map(|d| d as usize)
        })
        .collect::<Result<Vec<usize>>>()
        .with_context(|| format!("Making into an Vec failed"))
}

fn calculate_best_2_digit(digits: &[usize]) -> Result<usize, anyhow::Error> {
    // 987654321111111
    // walk through it from the right and compare each item
    // right most 1...best to right 21,31...91
    let mut best_digit = usize::MIN;
    let mut best_pair = usize::MIN;
    for (index, item) in digits.iter().rev().enumerate() {
        let current_digit = *item;
        if index != 0 {
            let candidate = current_digit * 10 + best_digit;
            if candidate > best_pair {
                best_pair = candidate;
            }
        }
        if current_digit > best_digit {
            best_digit = current_digit;
        }
    }
    Ok(best_pair)
}

fn calculate_best_12_digit(digits: &[usize]) -> Result<usize, anyhow::Error> {
    let length_of_output = 12;
    let n = digits.len();

    let mut result = Vec::new();
    let mut start_index = 0;
    let mut remaining = length_of_output;

    while remaining > 0 {
        let max_search_index = n - remaining;

        let mut best_digit = 0;
        let mut best_index = 0;

        for i in start_index..=max_search_index {
            if digits[i] > best_digit {
                best_digit = digits[i];
                best_index = i;
            }
        }

        result.push(best_digit);
        start_index = best_index + 1;
        remaining -= 1;
    }

    // Convert result digits to number
    let mut final_num = 0;
    for digit in result {
        final_num = final_num * 10 + digit;
    }

    Ok(final_num)
}

fn parse_input_lines(input: &str) -> Result<Vec<Vec<usize>>, anyhow::Error> {
    input
        .split_whitespace()
        .map(|i| parse_digits(i.trim()))
        .collect()
}

pub fn day3_partone(input: &str) -> Result<usize, anyhow::Error> {
    let lines = parse_input_lines(input)?;
    let mut output = 0;
    for digits in lines {
        output += calculate_best_2_digit(&digits)?;
    }
    Ok(output)
}

pub fn day3_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let lines = parse_input_lines(input)?;
    let mut output = 0;
    for digits in lines {
        output += calculate_best_12_digit(&digits)?;
    }
    Ok(output)
}
