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
struct HuffmanBuilder {
    root: BinaryHeap<Reverse<Node>>,
}

#[derive(Debug)]
struct HuffmanTree {
    root: Node,
}

// Design note for the next steps:
//
// Right now this `Huffman` struct is really acting as a tree builder. The
// `BinaryHeap` is useful while we are constructing the Huffman tree: it lets us
// repeatedly remove the two lowest-frequency nodes and merge them into a new
// internal node.
//
// After `build_tree` finishes, though, the heap is no longer the main data
// structure we care about. At that point the heap should contain exactly one
// item: the root of the finished Huffman tree. For encoding, decoding, and
// writing a file header, the useful object is the tree root, not the heap.
//
// A cleaner mental model is:
//
//     frequencies -> tree builder -> Huffman tree -> code table -> encoded bytes
//
// That suggests separating the construction phase from the usable tree phase:
//
//     struct HuffmanBuilder {
//         heap: BinaryHeap<Reverse<Node>>,
//     }
//
//     struct HuffmanTree {
//         root: Node,
//     }
//
// Then the builder would consume itself and return a finished tree:
//
//     impl HuffmanBuilder {
//         fn build(self) -> Result<HuffmanTree, anyhow::Error> {
//             ...
//         }
//     }
//
// This avoids an awkward state where a `Huffman` value may or may not be ready
// to encode. With a separate `HuffmanTree`, methods like `encode`, `decode`,
// `code_table`, and `encode_header` can live on a type that only exists after a
// valid tree has been built.
//
// For step 4, the header must store enough information to recreate the tree
// during decoding. Two common choices:
//
// 1. Store the frequency table.
//    This fits the code we already have: read the header, rebuild the
//    `HashMap<u8, usize>`, rebuild the same Huffman tree, then decode.
//
//    Important caveat: if two bytes have the same frequency, tree construction
//    must be deterministic. Otherwise the encoder and decoder might build two
//    different, but equally valid, trees. If that happens, the compressed bits
//    will be interpreted incorrectly. A stable tie-breaker, such as ordering by
//    byte value or assigning stable node IDs, fixes this.
//
// 2. Store the tree itself.
//    This is often more robust because the decoder reconstructs the exact tree
//    shape that the encoder used. The tradeoff is that it requires tree
//    serialization: writing leaves/internal nodes in a format you can parse
//    later.
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
    pub fn build_tree(&mut self) -> Result<(), anyhow::Error> {
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
        Ok(())
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
        let mut huffman = HuffmanBuilder::new();
        huffman.insert(map);
        huffman.build_tree()?;
        let root = huffman.root.pop().unwrap().0;
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
        huffman.build_tree()?;
        let root = huffman.root.pop().unwrap().0;
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
