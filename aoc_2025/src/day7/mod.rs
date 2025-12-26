use std::collections::HashSet;

pub fn day7_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = input
        .split_whitespace()
        .map(|item| item.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
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
