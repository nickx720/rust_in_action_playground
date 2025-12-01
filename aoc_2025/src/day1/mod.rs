fn split_code(s: &str) -> Option<(char, i32)> {
    let mut chars = s.chars();
    let first = chars.next()?;
    let rest: String = chars.collect();
    let num: i32 = rest.parse().ok()?;
    Some((first, num))
}

// helper to calculate modulo
fn modulo_calculator_offset(num: i32) -> i32 {
    ((num % 100) + 100) % 100
}

fn parse_input(input: &str) -> Vec<Option<(char, i32)>> {
    input
        .split("\n")
        .filter(|item| !item.is_empty())
        .map(|item| {
            let item = item.trim();
            split_code(item)
        })
        .collect::<Vec<Option<(char, i32)>>>()
}

pub fn day1_partone(input: &str) -> Result<i32, anyhow::Error> {
    let input = parse_input(input);
    let mut pos = 50;
    let mut count = 0;
    for pair in input {
        if let Some((op, code)) = pair {
            let dist = modulo_calculator_offset(code);
            pos = match op {
                'L' => modulo_calculator_offset(pos - dist),
                'R' => modulo_calculator_offset(pos + dist),
                _ => anyhow::bail!("unknown direction"),
            };
            if pos == 0 {
                count += 1;
            }
        } else {
            anyhow::bail!("Whoops");
        }
    }
    Ok(count)
}

pub fn day1_parttwo(input: &str) -> Result<i32, anyhow::Error> {
    let input = parse_input(input);
    let mut pos = 50;
    let mut count = 0;

    for pair in input {
        if let Some((op, code)) = pair {
            let dist = code.abs();

            let full_laps = dist / 100;
            count += full_laps;

            let rem = dist % 100;

            pos = match op {
                'L' => {
                    if pos > 0 && rem >= pos {
                        count += 1;
                    }
                    modulo_calculator_offset(pos - rem)
                }
                'R' => {
                    if pos + rem >= 100 {
                        count += 1;
                    }
                    modulo_calculator_offset(pos + rem)
                }
                _ => anyhow::bail!("unknown direction"),
            };
        } else {
            anyhow::bail!("Whoops");
        }
    }

    Ok(count)
}
