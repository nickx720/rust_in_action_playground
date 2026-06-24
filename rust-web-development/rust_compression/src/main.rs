use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
    fs::{self, File},
    io::{Read, Write},
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
    pub fn left(&self) -> Option<Box<Node>> {
        match self {
            Node::Internal { freq, left, right } => Some(left.clone()),
            _ => None,
        }
    }
    pub fn right(&self) -> Option<Box<Node>> {
        match self {
            Node::Internal { freq, left, right } => Some(right.clone()),
            _ => None,
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
    pub fn gen_prefix_table(&mut self) -> HashMap<u8, (Vec<u8>, usize)> {
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
        let mut code_path: HashMap<u8, (Vec<u8>, usize)> = HashMap::new();
        let mut stack: Vec<(Node, Vec<u8>)> = Vec::new();
        stack.push((self.root.clone(), Vec::new()));
        while let Some((node, table)) = stack.pop() {
            match node {
                Node::Leaf { byte, freq } => {
                    code_path.insert(byte, (table.to_vec(), freq));
                }
                Node::Internal {
                    freq: _,
                    left,
                    right,
                } => {
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
        code_path
    }
    pub fn decode(self, byte_stream: &[u8]) -> Vec<u8> {
        // walk the tree, till it finds a left, emit that feafs byte
        // reset current node back to root
        let mut output: Vec<u8> = Vec::new();
        let mut current = Box::new(self.root.clone());
        for bit in byte_stream {
            if *bit == 0 {
                // go left
                if let Some(nodee) = current.left() {
                    current = nodee;
                }
            }
            if *bit == 1 {
                // go right
                if let Some(nodee) = current.right() {
                    current = nodee;
                }
            }
            if current.is_leaf() {
                match *current {
                    Node::Leaf { byte, freq: _ } => {
                        output.push(byte);
                        // * keeps the same box, but updates the value inside
                        *current = self.root.clone();
                    }
                    _ => panic!("Illegal variant for current is leaf"),
                }
            }
            // if current is a leaf, return byte and reset current
        }

        output
    }
}

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
            root: self.root.pop().expect("No items in the tree").0,
        };
        Ok(tree)
    }
}

fn encode(
    prefix_table: HashMap<u8, (Vec<u8>, usize)>,
    source: &String,
    target: &String,
) -> Result<(), anyhow::Error> {
    let file = fs::canonicalize(source)?;
    let mut file = File::open(file)?;
    let mut buf = [0u8; 1024];
    let mut out_bytes: Vec<u8> = Vec::new();
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let data = &buf[..n];
        for key in data {
            if let Some(value) = prefix_table.get(key) {
                out_bytes.extend(value.0.as_slice());
            }
        }
    }
    let mut header = String::new();
    for (key, value) in prefix_table {
        let output = format!("{}:{}\n", key, value.1);
        header.push_str(&output);
    }
    let header_length = header.len() as u32;
    let mut file = File::create(target)?;
    file.write_all(&header_length.to_le_bytes())?;
    file.write_all(header.as_bytes())?;
    file.write_all(&out_bytes)?;
    Ok(())
}

fn decode(source: &String, target: &String) -> Result<(), anyhow::Error> {
    let file = fs::canonicalize(source)?;
    let mut file = File::open(file)?;
    let mut buf = [0u8; 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let data = &buf[..n];
        let arr: [u8; 4] = data[0..4].try_into().unwrap();
        let length = u32::from_le_bytes(arr);
        let text = std::str::from_utf8(&data[4..4 + length as usize])?;
        let mut encoded_output = HashMap::new();
        text.trim().split('\n').for_each(|item| {
            let key_value: Vec<&str> = item.split(':').collect();
            let key = key_value[0].parse::<u8>().unwrap();
            let value = key_value[1].parse::<usize>().unwrap();
            encoded_output.insert(key, value);
        });
        let mut huffman = HuffmanBuilder::new();
        // README Step 6 starts here: rebuild the decoding structure from the header.
        //
        // Be careful: the header currently stores only byte frequencies. That is enough
        // to build a valid Huffman tree, but not necessarily the exact same tree that
        // encode used when multiple bytes have the same frequency.
        //
        // Before changing decode's tree walk, first make sure this reconstruction step
        // is deterministic. Either store enough header data to recreate the exact codes,
        // or make HuffmanBuilder break frequency ties in a stable way so encode and
        // decode build the same left/right tree from the same frequency table.
        //
        // Huffman construction repeatedly combines the two lowest-frequency nodes.
        // Highest-frequency bytes usually get shorter codes because they survive longer,
        // closer to the root. When two nodes have the same frequency, however, Huffman
        // does not define which one must be chosen first. That choice matters here:
        // build_tree assigns the first popped node to `left` and the second to `right`,
        // and gen_prefix_table turns left into 0 and right into 1.
        //
        // So the practical question is: when frequencies tie, can encode and decode
        // guarantee the same node is popped first every time?
        huffman.insert(encoded_output.clone());
        let tree = huffman.build_tree()?;
        let huffman_bytes = &data[4 + length as usize..];
        let output = tree.decode(huffman_bytes);
        fs::write(target, output)?;
    }
    Ok(())
}

fn valid_file_path(items: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
    let mut huffman = HuffmanBuilder::new();
    match items.collect::<Vec<String>>().as_slice() {
        [action, source, target] => {
            if action.to_lowercase() == "encode" {
                let file = fs::canonicalize(source)?;
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
                let mut tree = huffman.build_tree()?;
                let prefix_table = tree.gen_prefix_table();
                encode(prefix_table, source, target)?;
            }
            if action.to_lowercase() == "decode" {
                decode(source, target)?;
            }
        }
        _ => panic!("Unsupported action"),
    }
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1);
    dbg!(&args);
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
