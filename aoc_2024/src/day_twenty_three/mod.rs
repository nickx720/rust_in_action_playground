use std::collections::{HashMap, HashSet};

pub mod part_one;
pub mod part_two;

fn make_graph(input: &str) -> (HashSet<&str>, HashMap<&str, Vec<&str>>) {
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let (n1, n2) = (parts[0], parts[1]);
        if !nodes.contains(&n1) {
            nodes.insert(n1);
        }
        if !nodes.contains(n2) {
            nodes.insert(n2);
        }
        edges.entry(n1).or_insert(vec![]).push(n2);
        edges.entry(n2).or_insert(vec![]).push(n1);
    }
    (nodes, edges)
}
