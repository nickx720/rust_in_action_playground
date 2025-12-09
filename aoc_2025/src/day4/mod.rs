const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split_whitespace()
        .map(|item| {
            item.split("")
                .map(|item| item.trim())
                .filter(|item| !item.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
}

pub fn day4_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = parse_input(input);
    // loop through each item, and check if either side has < 4
    let mut output = 0;
    let width = input.len() as isize;
    let height = input[0].len() as isize;
    for index in 0..width {
        for second_index in 0..height {
            if input[index as usize][second_index as usize] == '.'.to_string() {
                continue;
            }
            let mut count = 0;
            for (x, y) in DIRS {
                let nx = index + x;
                let ny = second_index + y;
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    if input[nx as usize][ny as usize] == "@".to_string() {
                        count += 1;
                    }
                }
            }
            if count <= 3 {
                output += 1;
            }
        }
    }
    Ok(output)
}

pub fn day4_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    todo!()
}
