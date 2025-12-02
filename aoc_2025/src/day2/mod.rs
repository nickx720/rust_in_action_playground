use std::collections::HashMap;

use anyhow::{Context, Result};

fn all_values_equal(count: &HashMap<&str, i32>) -> bool {
    let mut iter = count.values();
    let Some(first) = iter.next() else {
        return true;
    };
    iter.all(|v| v == first)
}

pub fn day2_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input: Vec<(usize, usize)> = input
        .split(",")
        .map(|item| {
            let mut parts = item.trim().split("-");
            let start = parts
                .next()
                .context("range missing start value")?
                .parse::<usize>()
                .with_context(|| format!("invalid start in '{item}'"))?;
            let end = parts
                .next()
                .context("range missing end value")?
                .parse::<usize>()
                .with_context(|| format!("invalid end value '{item}'"))?;
            Ok((start, end))
        })
        // collect::<Result<Vec<_>>>()? gathers the Ok tuples into a Vec and
        // short-circuits as soon as an Err appears.
        .collect::<Result<Vec<_>>>()?;
    let mut output = 0;
    for (start, end) in input {
        for item in start..=end {
            let string_item = item.to_string();
            let mut count = HashMap::new();
            for item in string_item.split("") {
                if count.get(item).is_none() {
                    count.insert(item, 1);
                } else {
                    let count_now = count.get(item).unwrap();
                    count.insert(item, *count_now + 1);
                }
            }
            if all_values_equal(&count) {
                dbg!(item);
                output += item;
            }
        }
    }
    Ok(output)
}
