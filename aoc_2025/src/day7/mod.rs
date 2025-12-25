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
    Ok(split_count)
}
