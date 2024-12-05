use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufReader, Read},
};
pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut buffer = BufReader::new(file);
    let mut contents = String::new();
    buffer.read_to_string(&mut contents)?;
    let input: Vec<&str> = contents
        .split("\n\n")
        .map(|item| item.trim())
        .collect::<Vec<&str>>();
    let (edges, queries) = (input[0], input[1]);
    let mut counted_set: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut counted_queue: HashMap<usize, Vec<usize>> = HashMap::new();
    for item in edges.split_whitespace() {
        let split_item = item.split("|").collect::<Vec<&str>>();
        let (x, y) = (split_item[0], split_item[1]);
        let (x, y) = (x.parse::<usize>()?, y.parse::<usize>()?);
        if !counted_set.contains_key(&y) || counted_set.is_empty() {
            counted_set.insert(y, vec![x]);
        } else {
            let current_items = counted_set.get_mut(&y).expect("No Vec found");
            current_items.push(x);
        }
        if !counted_queue.contains_key(&x) || counted_queue.is_empty() {
            counted_queue.insert(x, vec![y]);
        } else {
            let current_items = counted_queue.get_mut(&x).expect("No Vec found");
            current_items.push(y);
        }
    }
    let mut output = 0;
    for item in queries.split_whitespace() {
        let mut add_flag = true;
        let collected_items = item
            .split(",")
            .map(|item| item.parse::<usize>().expect("Parsing error"))
            .collect::<Vec<usize>>();
        for (i, &x) in collected_items.iter().enumerate() {
            for (j, &y) in collected_items.iter().enumerate() {
                if i < j {
                    let item = counted_set.get(&x);
                    match item {
                        Some(val) => {
                            if val.contains(&y) {
                                add_flag = false;
                            }
                        }
                        None => continue,
                    }
                }
            }
        }
        if !add_flag {
            let mut good = Vec::new();
            let mut queue = VecDeque::new();
            let mut collection: HashMap<usize, usize> = collected_items
                .iter()
                .map(|&v| {
                    let count = counted_set.get(&v).map_or(0, |set| {
                        set.iter()
                            .cloned()
                            .collect::<HashSet<usize>>()
                            .intersection(&collected_items.iter().cloned().collect())
                            .count()
                    });
                    (v, count)
                })
                .collect();
            for item in &collected_items {
                if *collection.get(&item).unwrap_or(&0) == 0 {
                    queue.push_back(item);
                }
            }
            while let Some(item) = queue.pop_front() {
                good.push(item);
                if let Some(adjacent) = counted_queue.get(item) {
                    for new_item in adjacent {
                        if let Some(count) = collection.get_mut(new_item) {
                            *count -= 1;
                            if *count == 0 {
                                queue.push_back(new_item);
                            }
                        }
                    }
                }
            }
            output += good[good.len() / 2]
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_five_part_two() -> Result<(), Box<dyn Error>> {
        let output = part_two("./assets/day_five/sample.txt")?;
        assert_eq!(123, output);
        Ok(())
    }
}
