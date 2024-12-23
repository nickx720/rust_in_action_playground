use std::{collections::HashSet, error::Error, fs::File, io::Read};

use crate::day_twenty_three::make_graph;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    let (_, edges) = make_graph(&input);
    let mut three_groups = HashSet::new();
    for (node, neighbours) in &edges {
        for neighbour in neighbours {
            let others = edges.get(neighbour).expect("No neighbour found");
            for other in others {
                if neighbours.contains(other) {
                    let mut group = vec![node, neighbour, other];
                    group.sort();
                    if !three_groups.contains(&group) {
                        three_groups.insert(group);
                    }
                }
            }
        }
    }
    let output = three_groups
        .iter()
        .filter(|group| group.iter().any(|n| n.starts_with('t')))
        .count();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_twenty_three_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_three/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 7);
        Ok(())
    }
}
