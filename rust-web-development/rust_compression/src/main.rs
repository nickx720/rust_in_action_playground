use std::{
    collections::{BinaryHeap, HashMap},
    env,
    fs::{self, File},
    io::Read,
};

fn frequency_counter(data: &[u8], map: &mut HashMap<u8, usize>) -> Result<(), anyhow::Error> {
    for word in data.iter() {
        let word = word.to_ascii_lowercase();
        map.entry(word.to_owned())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    Ok(())
}

#[derive(Debug)]
struct Huffman {
    heap: BinaryHeap<(u8, usize)>,
}

impl Huffman {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn insert(&mut self, map: HashMap<u8, usize>) {
        for val in map.into_iter() {
            self.heap.push(val);
        }
    }
}

fn valid_file_path(items: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
    let mut huffman = Huffman::new();
    for arg in items {
        let file = fs::canonicalize(arg)?;
        let mut file = File::open(file)?;
        let mut buf = [0u8; 1024];
        let mut map = HashMap::new();
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            let data = &buf[..n];
            frequency_counter(data, &mut map)?;
        }
        huffman.insert(map);
    }
    dbg!(&huffman);
    for (item, _) in huffman.heap.into_vec() {
        dbg!(str::from_utf8(&[item]).unwrap());
    }
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1);
    valid_file_path(args)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_counter() -> Result<(), anyhow::Error> {
        let string_to_check = "aabbc".as_bytes();
        let mut map = HashMap::new();
        frequency_counter(string_to_check, &mut map)?;
        let output = HashMap::from([(b'a', 2usize), (b'b', 2usize), (b'c', 1usize)]);
        assert_eq!(map, output);
        Ok(())
    }
}
