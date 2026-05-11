use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
    fs::{self, File},
    io::Read,
};

fn frequency_counter(data: &[u8], map: &mut HashMap<u8, usize>) -> Result<(), anyhow::Error> {
    for word in data.iter() {
        map.entry(word.to_owned())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    Ok(())
}
// An enum models the valid states directly: a Huffman node is either a leaf or an internal node, never both.
#[derive(Debug, PartialEq, Eq)]
enum Node {
    Leaf {
        byte: u8,
        freq: usize,
    },
    Internal {
        freq: usize,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    fn freq(&self) -> usize {
        match self {
            Node::Leaf { freq, .. } => *freq,
            Node::Internal { freq, .. } => *freq,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Huffman {
    heap: BinaryHeap<Reverse<Node>>,
}

impl Huffman {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn insert(&mut self, map: HashMap<u8, usize>) {
        for (byte, freq) in map.into_iter() {
            let node = Node::Leaf { byte, freq };
            self.heap.push(Reverse(node));
        }
    }
    pub fn build_tree(&mut self) -> Result<(), anyhow::Error> {
        while self.heap.len() > 1 {
            let left = self.heap.pop().ok_or(anyhow::anyhow!("Node not found"))?.0;
            let right = self.heap.pop().ok_or(anyhow::anyhow!("Node not found"))?.0;
            let left_tree_freq = left.freq();
            let right_tree_freq = right.freq();
            let freq = left_tree_freq + right_tree_freq;
            let new_node = Node::Internal {
                freq,
                left: Box::new(left),
                right: Box::new(right),
            };
            self.heap.push(Reverse(new_node));
        }
        Ok(())
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
    huffman.build_tree()?;
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
    #[test]
    fn validate_huffman_build() -> Result<(), anyhow::Error> {
        let mut map = HashMap::new();
        map.insert(b'a', 5);
        map.insert(b'b', 2);
        map.insert(b'c', 1);
        let mut huffman = Huffman::new();
        huffman.insert(map);
        huffman.build_tree()?;
        let root = huffman.heap.pop().unwrap().0;
        match root {
            Node::Internal { freq, left, right } => {
                assert_eq!(freq, 8);
                assert_eq!(left.freq(), 3);
                assert_eq!(right.freq(), 5);
            }
            _ => panic!("Test failed"),
        }
        Ok(())
    }
    #[test]
    fn validate_huffman_build_open_dsa() -> Result<(), anyhow::Error> {
        let mut map = HashMap::new();
        map.insert(b'c', 32);
        map.insert(b'd', 42);
        map.insert(b'e', 120);
        map.insert(b'k', 7);
        map.insert(b'l', 42);
        map.insert(b'm', 24);
        map.insert(b'u', 37);
        map.insert(b'z', 2);
        let mut huffman = Huffman::new();
        huffman.insert(map);
        huffman.build_tree()?;
        let root = huffman.heap.pop().unwrap().0;
        match root {
            Node::Internal { freq, left, right } => {
                assert_eq!(freq, 306);
                assert_eq!(left.freq(), 120);
                match *right {
                    Node::Internal { freq, left, right } => {
                        assert_eq!(freq, 186);
                        assert_eq!(left.freq(), 79);
                        assert_eq!(right.freq(), 107);
                    }
                    _ => panic!("Test failed"),
                }
            }
            _ => panic!("Test failed"),
        }
        Ok(())
    }
}
