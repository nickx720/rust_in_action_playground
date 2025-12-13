use anyhow::{anyhow, Context, Result};

fn split_blocks(input: &str) -> Result<(&str, &str)> {
    input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Bad input format"))
}

fn parse_ranges(block: &str) -> Result<Vec<(u64, u64)>> {
    block
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
        .collect()
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    let (mut s, mut e) = ranges[0];
    for (a, b) in ranges.into_iter().skip(1) {
        if a <= e.saturating_add(1) {
            e = e.max(b);
        } else {
            merged.push((s, e));
            (s, e) = (a, b);
        }
    }
    merged.push((s, e));
    merged
}

fn parse_items(block: &str) -> Result<Vec<u64>> {
    block
        .split_whitespace()
        .map(|s| s.parse::<u64>().context("Conversion failed"))
        .collect()
}

pub fn day5_partone(input: &str) -> Result<usize, anyhow::Error> {
    let (ranges_block, items_block) = split_blocks(input)?;

    let merged = merge_ranges(parse_ranges(ranges_block)?);
    let items = parse_items(items_block)?;

    let mut count = 0usize;
    for item in items {
        // Disjoint merged intervals allow early exit.
        for (start, end) in &merged {
            if item < *start {
                break;
            }
            if item <= *end {
                count += 1;
                break;
            }
        }
    }
    Ok(count)
}

pub fn day5_parttwo(input: &str) -> Result<usize> {
    let (ranges_block, _) = split_blocks(input)?;
    let merged = merge_ranges(parse_ranges(ranges_block)?);

    let mut total: u128 = 0;
    for (s, e) in merged {
        total += (e as u128) - (s as u128) + 1;
    }

    Ok(total as usize)
}
