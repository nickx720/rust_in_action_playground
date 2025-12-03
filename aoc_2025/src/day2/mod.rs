use anyhow::{Context, Result};

fn invalid_id(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    };
    let len = s.len() / 2;
    let (first, second) = (
        s[0..len].parse::<usize>().unwrap(),
        s[len..].parse::<usize>().unwrap(),
    );
    if first != second {
        return false;
    }
    true
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
            if invalid_id(&string_item) {
                output += item;
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_invalid() {
        let item = "222222";
        let output = invalid_id(item);
        assert_eq!(true, output);
    }
}
