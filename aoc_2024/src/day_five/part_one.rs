use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
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
        if add_flag {
            output += collected_items[collected_items.len() / 2];
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_five_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_five/sample.txt";
        let output = part_one(path)?;
        assert_eq!(143, output);
        Ok(())
    }
}
