use std::collections::{HashSet, VecDeque};

pub mod part_one;
pub mod part_two;

type Byte = (i32, i32);
type TInput = Vec<Byte>;

fn neighbours(byte: &Byte) -> Vec<Byte> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(dx, dy)| (byte.0 + *dx, byte.1 + *dy))
        .collect()
}

fn explore_at_time(bytes: &TInput, time: usize) -> Option<u32> {
    let (x, y) = if bytes.len() > 1000 { (71, 71) } else { (7, 7) };
    let mut mem: HashSet<Byte> = (0..x)
        .flat_map(|item| (0..y).map(move |second_item| (item, second_item)))
        .collect();
    for byte in bytes.iter().take(time) {
        mem.remove(byte);
    }
    let mut to_explore = VecDeque::from([((0, 0), 0)]);
    let mut explored = HashSet::new();
    while let Some((cur, dis)) = to_explore.pop_back() {
        if !explored.insert(cur) {
            continue;
        }
        if cur == (x - 1, y - 1) {
            return Some(dis);
        }

        for neighbour in neighbours(&cur) {
            if !explored.contains(&neighbour) & mem.contains(&neighbour) {
                to_explore.push_front((neighbour, dis + 1));
            }
        }
    }
    None
}
