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
    pub fn encode() -> Vec<u8> {
        // Depth-first search (DFS) explores one complete path before trying
        // the next sibling path. On a tree, that usually means:
        //
        //     1. Visit the current node.
        //     2. Choose one child and keep going until that branch ends.
        //     3. Backtrack to the nearest unfinished node.
        //     4. Repeat until every reachable node has been visited.
        //
        // Huffman encoding is a natural DFS problem. Each edge records one bit
        // in the current path: conventionally left = 0 and right = 1. When the
        // walk reaches a leaf, that path is the prefix code for the leaf's byte.
        //
        // Example shape:
        //
        //          root
        //         /    \
        //       'a'    internal
        //              /      \
        //            'b'      'c'
        //
        // DFS paths would produce codes like:
        //
        //     a -> 0
        //     b -> 10
        //     c -> 11
        //
        // In code, DFS can be recursive or stack-based. For recursive Huffman
        // traversal, push a bit before descending into a child, record the path
        // at a leaf, then pop that bit while backtracking.
        //
        // Stack-based DFS pseudocode:
        //
        //     DFS(graph, start):
        //         create empty stack
        //         create empty visited set
        //
        //         push start onto stack
        //
        //         while stack is not empty:
        //             node = pop from stack
        //
        //             if node is already in visited:
        //                 continue
        //
        //             mark node as visited
        //             process node
        //
        //             for each neighbor of node:
        //                 if neighbor is not in visited:
        //                     push neighbor onto stack
        //
        // To match recursive DFS traversal order, push neighbors in reverse:
        //
        //     DFS(graph, start):
        //         stack = [start]
        //         visited = empty set
        //
        //         while stack is not empty:
        //             node = stack.pop()
        //
        //             if node in visited:
        //                 continue
        //
        //             visited.add(node)
        //             process(node)
        //
        //             for neighbor in reverse(graph[node]):
        //                 if neighbor not in visited:
        //                     stack.push(neighbor)
        let mut path: Vec<u8> = Vec::new();
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
    let tree = huffman.build_tree()?;
    dbg!(tree);
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
