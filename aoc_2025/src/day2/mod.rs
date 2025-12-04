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

fn parse_input(input: &str) -> Result<Vec<(usize, usize)>, anyhow::Error> {
    input
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
        .collect::<Result<Vec<_>>>()
        .with_context(|| format!("parsing failed"))
}

pub fn day2_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = parse_input(input)?;
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
// divisors come in pairs
fn divisors(n: usize) -> Vec<usize> {
    let mut output = Vec::new();
    let limit = (n as f64).sqrt() as usize;
    for i in 1..=limit {
        if n % i == 0 {
            output.push(i);
            // each divisor come in pairs so if n % i ==0 it means its pair is i != n / i
            if i != n / i {
                output.push(n / i)
            }
        }
    }
    output.sort_unstable();
    output
}

fn is_sequence(string: &str) -> bool {
    let len = string.len();
    let chars: Vec<char> = string.chars().collect();
    let divisors = divisors(len);
    // 12341234
    // 1,2,4,8
    //    dbg!(&divisors, string);
    for chunk in divisors {
        if len >= chunk * 2 {
            let first_chunk: String = chars[0..chunk].iter().collect();
            let mut all_match = true;
            for chunk_start in (chunk..len).step_by(chunk) {
                let second_chunk: String = chars[chunk_start..chunk_start + chunk].iter().collect();
                if second_chunk != first_chunk {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                return true;
            }
        }
    }
    false
}

pub fn day2_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let input = parse_input(input)?;
    let mut output = 0;
    for (start, end) in input {
        for item in start..=end {
            let string_item = item.to_string();
            if is_sequence(&string_item) {
                dbg!(item);
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
