pub fn day7_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = input
        .split_whitespace()
        .map(|item| item.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let rows = input.len();
    let cols = input[0].len();
    for i in 0..rows {
        for j in 0..cols {
            dbg!(input[i][j] as char);
        }
    }
    todo!()
}
