use std::{
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

#[derive(Debug)]
struct Huffman {
    heap: BinaryHeap<(u8, usize)>,
}

// Step 2 TODOs, based on OpenDSA Huffman Coding Trees:
// https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html
//
// Read these sections while implementing:
// - "Building Huffman Coding Trees"
// - the sample node types (`HuffLeafNode`, `HuffInternalNode`)
// - the sample `buildTree()` loop
//
// The important idea:
// - the heap is only the priority queue / work area
// - the Huffman tree is the value you are building inside that queue
//
// Suggested implementation steps:
//
// 1. Replace `(u8, usize)` with a real tree node type.
//    Start with something like:
//    - `Leaf { byte, freq }`
//    - `Internal { freq, left, right }`
//    OpenDSA uses separate leaf and internal node classes for this reason:
//    leaves store a symbol, internal nodes store child pointers.
//
// 2. Make the heap hold "partial Huffman trees", not raw frequency tuples.
//    At the beginning, each symbol/frequency pair becomes a one-node tree.
//    This matches OpenDSA's "create n initial Huffman trees, each a single leaf".
//
// 3. Make it a min-heap by frequency.
//    Huffman needs repeated access to the two *smallest* frequencies.
//    Rust's `BinaryHeap` is a max-heap by default, so use `Reverse`
//    or custom `Ord`/`PartialOrd` implementations.
//
// 4. Write a `build_tree` function.
//    The OpenDSA loop is:
//    - pop the smallest tree
//    - pop the next smallest tree
//    - create a new internal node with:
//      - left = first tree
//      - right = second tree
//      - freq = left.freq + right.freq
//    - push the merged tree back into the heap
//
// 5. Repeat until one tree remains.
//    That final remaining item is the Huffman tree root.
//    This is the actual output of step 2.
//
// 6. Sanity-check your result with a tiny example.
//    Example input frequencies:
//    - a: 5
//    - b: 2
//    - c: 1
//    Expected process:
//    - merge c(1) + b(2) -> parent(3)
//    - merge parent(3) + a(5) -> root(8)
//
// 7. Keep step 3 in mind while designing step 2.
//    In step 3 you will traverse the final tree:
//    - left edge => 0
//    - right edge => 1
//    That only works if step 2 preserves the left/right child structure.
//
// 8. For a lossless compressor, do not lowercase bytes while counting.
//    `A` and `a` must stay distinct if you want decode(encode(x)) == x.
//
// Practical checkpoint:
// - if your heap contains only `(byte, freq)`, you have not built the tree yet
// - if your heap contains nodes/trees and you merge two smallest until one
//   root remains, you are implementing step 2 correctly
//
// OpenDSA references for the exact algorithm and example node design:
// https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html
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
    //    for (item, _) in huffman.heap.into_vec() {
    //        dbg!(str::from_utf8(&[item]).unwrap());
    //    }
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
