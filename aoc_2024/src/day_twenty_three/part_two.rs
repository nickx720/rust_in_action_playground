use std::{error::Error, fs::File, io::Read};

use crate::day_twenty_three::make_graph;

pub fn part_two(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(&path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let (nodes, edges) = make_graph(&input);
    let mut largest_group = vec![];
    for node in nodes {
        let mut group = vec![node];
        if let Some(neighbours) = edges.get(node) {
            for neighbour in neighbours {
                if group
                    .iter()
                    .all(|n| edges.get(neighbour).expect("No edge found").contains(n))
                {
                    group.push(neighbour);
                }
            }
            group.sort();
            if group.len() > largest_group.len() {
                largest_group = group;
            }
        }
    }
    let output = largest_group
        .into_iter()
        .map(|item| item.to_owned())
        .collect::<Vec<String>>()
        .join(",");
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_twenty_three_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_three/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, "co,de,ka,ta");
        Ok(())
    }
}
