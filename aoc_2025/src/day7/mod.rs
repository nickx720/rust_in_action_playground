use std::collections::HashSet;

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>, anyhow::Error> {
    Ok(input
        .split_whitespace()
        .map(|item| item.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>())
}

pub fn day7_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = parse_input(input)?;
    let rows = input.len();
    let cols = input[0].len();
    let mut set = HashSet::new();
    if let Some(idx) = input[0].iter().position(|&x| x == b'S') {
        set.insert(idx);
    } else {
        anyhow::bail!("Couldn't find starting beam ");
    }
    let mut split_count = 0usize;
    for row in 0..rows - 1 {
        let mut next_active = HashSet::new();
        for &col in set.iter() {
            if col >= cols {
                continue;
            }
            let item = input[row + 1][col];
            if item == b'.' {
                next_active.insert(col);
            } else if item == b'^' {
                split_count += 1;
                if col > 0 {
                    next_active.insert(col - 1);
                }
                if col + 1 < cols {
                    next_active.insert(col + 1);
                }
            }
        }
        set = next_active;
        if set.is_empty() {
            break;
        }
    }
    Ok(split_count)
}

pub fn day7_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let input = parse_input(input)?;
    let rows = input.len();
    let cols = input[0].len();
    let mut start_pos: Option<(usize, usize)> = None;
    for (r, row) in input.iter().enumerate() {
        if let Some(c) = row.iter().position(|&x| x == b'S') {
            start_pos = Some((r, c));
            break;
        }
    }
    let (start_r, start_c) =
        start_pos.ok_or_else(|| anyhow::anyhow!("Couldn't find starting beam"))?;

    let mut timelines_at_col = vec![0usize; cols];
    timelines_at_col[start_c] = 1;

    for r in start_r..rows.saturating_sub(1) {
        let mut next = vec![0usize; cols];

        for c in 0..cols {
            let k = timelines_at_col[c];
            if k == 0 {
                continue;
            }

            let cell = input[r + 1][c];
            if cell == b'.' {
                next[c] += k;
            } else if cell == b'^' {
                if c > 0 {
                    next[c - 1] += k;
                }
                if c + 1 < cols {
                    next[c + 1] += k;
                }
            }
        }

        timelines_at_col = next;
        if timelines_at_col.iter().sum::<usize>() == 0 {
            break;
        }
    }

    Ok(timelines_at_col.iter().sum())
}
