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
#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf { .. } => true,
            Node::Internal { .. } => false,
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
struct HuffmanBuilder {
    root: BinaryHeap<Reverse<Node>>,
}

#[derive(Debug)]
struct HuffmanTree {
    root: Node,
}

impl HuffmanTree {
    pub fn encode(&mut self) -> Vec<u8> {
        // This traversal is for building the Huffman code table.
        //
        // A code table answers the question:
        //
        //     "For this byte, what bits should I write?"
        //
        // Its shape is:
        //
        //     byte -> path
        //
        // For example:
        //
        //     b'a' -> [0]
        //     b'b' -> [1, 0]
        //     b'c' -> [1, 1]
        //
        // Stack-based DFS works if each stack item stores both:
        //
        //     1. the node to process
        //     2. the path taken from the root to that node
        //
        //     stack = [(root, [])]
        //
        //     while stack is not empty:
        //         node, path = stack.pop()
        //
        //         if node is a leaf:
        //             record byte -> path in the code table
        //
        //         if node is internal:
        //             push right child with path + [1]
        //             push left child with path + [0]
        //
        // "Record byte" means inserting into something like:
        //
        //     HashMap<u8, Vec<u8>>
        //
        // A tree does not need a visited set because there are no cycles.
        // The path is the important state: every left edge adds 0, and every
        // right edge adds 1. When the traversal reaches a leaf, that path is
        // the complete Huffman code for the leaf's byte.
        let mut code_path: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut stack: Vec<(Node, Vec<u8>)> = Vec::new();
        stack.push((self.root.clone(), Vec::new()));
        while let Some((node, table)) = stack.pop() {
            match node {
                Node::Leaf { byte, .. } => {
                    code_path.insert(byte, table.to_vec());
                }
                Node::Internal { freq, left, right } => {
                    // figure out path
                    {
                        let mut right_table = table.clone();
                        right_table.push(1);
                        stack.push((*right, right_table));
                    }
                    {
                        let mut left_table = table.clone();
                        left_table.push(0);
                        stack.push((*left, left_table));
                    }
                }
            }
        }
        dbg!(code_path);
        todo!()
    }

    pub fn decode() -> Self {
        todo!()
    }
}
//
// A practical learning path:
//
//     - First, make `build_tree` return a finished root/tree instead of leaving
//       the root inside the heap.
//     - Then generate the prefix-code table by walking the tree:
//       left edge = 0, right edge = 1, leaf path = byte code.
//     - For step 4, start with a frequency-table header because it builds on
//       the work already done.
//     - Before relying on that header format, make the tree-building order
//       deterministic for equal frequencies.
//
impl HuffmanBuilder {
    pub fn new() -> Self {
        Self {
            root: BinaryHeap::new(),
        }
    }
    pub fn insert(&mut self, map: HashMap<u8, usize>) {
        for (byte, freq) in map.into_iter() {
            let node = Node::Leaf { byte, freq };
            self.root.push(Reverse(node));
        }
    }
    pub fn build_tree(&mut self) -> Result<HuffmanTree, anyhow::Error> {
        while self.root.len() > 1 {
            let left = self.root.pop().ok_or(anyhow::anyhow!("Node not found"))?.0;
            let right = self.root.pop().ok_or(anyhow::anyhow!("Node not found"))?.0;
            let left_tree_freq = left.freq();
            let right_tree_freq = right.freq();
            let freq = left_tree_freq + right_tree_freq;
            let new_node = Node::Internal {
                freq,
                left: Box::new(left),
                right: Box::new(right),
            };
            self.root.push(Reverse(new_node));
        }
        let tree = HuffmanTree {
            root: self.root.pop().unwrap().0,
        };
        Ok(tree)
    }
}

fn valid_file_path(items: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
    let mut huffman = HuffmanBuilder::new();
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
    let mut tree = huffman.build_tree()?;
    tree.encode();
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
        let mut huffman = HuffmanBuilder::new();
        huffman.insert(map);
        let tree = huffman.build_tree()?;
        let root = tree.root;
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
        let mut huffman = HuffmanBuilder::new();
        huffman.insert(map);
        let tree = huffman.build_tree()?;
        let root = tree.root;
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
