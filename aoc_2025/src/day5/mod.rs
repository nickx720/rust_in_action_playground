use anyhow::{Context, Result, anyhow};

fn process_input<T>(item: Option<T>) -> Result<Vec<String>, anyhow::Error>
where
    T: AsRef<str>,
{
    let item = item.ok_or_else(|| anyhow::anyhow!("Parsing failed"))?;
    Ok(item
        .as_ref()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect())
}

pub fn day5_partone(input: &str) -> Result<usize, anyhow::Error> {
    let mut count = 0;
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut input = input.iter();
    let ranges: Vec<(usize, usize)> = process_input(input.next())?
        .into_iter()
        .map(|item| {
            Ok::<Vec<(usize, usize)>, anyhow::Error>(
                item.split("-")
                    .map(|value| {
                        value
                            .parse::<usize>()
                            .with_context(|| format!("Conversion failed"))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .chunks(2)
                    .map(|chunk| (chunk[0], chunk[1]))
                    .collect::<Vec<(usize, usize)>>(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    let items: Vec<usize> = process_input(input.next())?
        .iter()
        .map(|item| {
            item.parse::<usize>()
                .with_context(|| format!("Conversaion failed"))
        })
        .collect::<Result<Vec<_>>>()?;
    for item in items {
        for (min, max) in &ranges {
            if item >= *min && item <= *max {
                count += 1;
                break;
            }
        }
    }
    Ok(count)
}

pub fn day5_parttwo(input: &str) -> Result<usize> {
    let first = input
        .split_once("\n\n")
        .map(|(a, _)| a)
        .with_context(|| format!("Parsing failed"))?;

    let mut ranges: Vec<(u64, u64)> = first
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l
                .split_once('-')
                .ok_or_else(|| anyhow!("Bad range line: {:?}", l))?;
            let a: u64 = a.parse().context("Parse failed")?;
            let b: u64 = b.parse().context("Parse failed")?;
            Ok::<_, anyhow::Error>(if a <= b { (a, b) } else { (b, a) })
        })
        .collect::<Result<_>>()?;

    ranges.sort_unstable();

    let mut total: u128 = 0;
    let (mut s, mut e) = ranges[0];

    for (a, b) in ranges.into_iter().skip(1) {
        if a <= e.saturating_add(1) {
            e = e.max(b);
        } else {
            total += (e as u128) - (s as u128) + 1;
            (s, e) = (a, b);
        }
    }
    total += (e as u128) - (s as u128) + 1;

    Ok(total as usize)
}
