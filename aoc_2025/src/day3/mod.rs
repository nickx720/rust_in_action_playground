fn calculate_best(input: &str) -> Option<u32> {
    // 987654321111111
    // walk through it from the right and compare each item
    // right most 1...best to right 21,31...91
    let mut best_fit = None;
    for item in input.chars().rev() {
        if best_fit.is_none() {
            best_fit = item.to_digit(10);
            continue;
        }
    }
    best_fit
}

pub fn day3_partone(input: &str) -> Result<u32, anyhow::Error> {
    let input = input
        .split_whitespace()
        .map(|i| i.trim())
        .collect::<Vec<&str>>();
    let mut output = 0;
    for item in input {
        if calculate_best(item).is_some() {
            output += calculate_best(item).unwrap();
        }
    }
    Ok(output)
}
