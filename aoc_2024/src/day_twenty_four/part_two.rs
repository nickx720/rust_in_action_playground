use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum OP {
    OR,
    AND,
    XOR,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Gate {
    in1: String,
    in2: String,
    out: String,
    op: OP,
}

#[derive(Debug, Clone)]
struct Machine {
    inputs: HashMap<String, u8>,
    finished: HashSet<Gate>,
    unfinished: HashSet<Gate>,
}

impl Machine {
    fn new(gates: Vec<Gate>, inputs: HashMap<String, u8>) -> Self {
        Machine {
            inputs,
            finished: HashSet::new(),
            unfinished: gates.into_iter().collect(),
        }
    }

    fn is_output(label: &String) -> bool {
        label.starts_with('z')
    }

    fn waiting_outputs(&self) -> HashSet<String> {
        self.unfinished
            .iter()
            .filter_map(|g| Self::is_output(&g.out).then_some(g.out.clone()))
            .collect()
    }

    fn calc(inputs: &HashMap<String, u8>, gate: &Gate) -> Option<u8> {
        let in1 = inputs.get(&gate.in1)?;
        let in2 = inputs.get(&gate.in2)?;
        match gate.op {
            OP::OR => Some(in1 | in2),
            OP::AND => Some(in1 & in2),
            OP::XOR => Some(in1 ^ in2),
        }
    }
    fn output(&self) -> u64 {
        self.inputs
            .iter()
            .filter_map(|(l, v)| Self::is_output(l).then_some((l, v)))
            .sorted_by(|a, b| a.0.cmp(b.0))
            .rev()
            .fold(0, |acc, x| (acc * 2) + *x.1 as u64)
    }

    fn tick(&mut self) {
        let mut f = Vec::new();
        let mut i = Vec::new();
        let mut u = Vec::new();
        for g in self.unfinished.drain() {
            if let Some(v) = Self::calc(&self.inputs, &g) {
                i.push((g.out.clone(), v));
                f.push(g);
            } else {
                u.push(g);
            }
        }
        self.inputs.extend(i);
        self.finished.extend(f);
        self.unfinished = u.into_iter().collect();
    }
}
fn parse(input: String) -> Result<Machine, Box<dyn Error>> {
    let (ws, gs) = input.split_once("\n\n").expect("Something went wrong");
    let wires: Result<HashMap<String, u8>, Box<dyn Error>> = Ok(ws).and_then(|sw| {
        sw.lines()
            .map(|sl| {
                let (l, v) = sl.split_once(": ").expect("split failed");
                let label = l.to_string();
                let value = v.parse().expect("parsing error");
                Ok((label, value))
            })
            .collect()
    });
    let gates: Result<Vec<Gate>, Box<dyn Error>> = Ok(gs).and_then(|sg| {
        sg.lines()
            .map(|sl| {
                let (lhs, rhs) = sl.split_once(" -> ").expect("split failed");
                let tokens = lhs.split_whitespace().collect_vec();
                Ok(Gate {
                    in1: tokens[0].to_string(),
                    in2: tokens[2].to_string(),
                    out: rhs.to_string(),
                    op: match tokens[1] {
                        "OR" => OP::OR,
                        "AND" => OP::AND,
                        "XOR" => OP::XOR,
                        _ => panic!("Something went wrong"),
                    },
                })
            })
            .collect()
    });
    Ok(Machine::new(gates.unwrap(), wires.unwrap()))
}

fn solve(m: &Machine) -> String {
    let gs: HashMap<(&String, &String, OP), &Gate> = m
        .unfinished
        .iter()
        .map(|g| ((&g.in1, &g.in2, g.op), g))
        .collect();
    let mut swapped = Vec::new();
    for g in &m.unfinished {
        if swapped.contains(&&g.out) {
            continue;
        }
        //g is OR/AND and ouputs zXX - swap with 2nd xor from xXX yXX -> xor ->xor
        if g.op != OP::XOR && g.out.starts_with('z') {
            let idx = g.out.strip_prefix('z').unwrap();
            if idx == "45" {
                continue;
            } //last one is fine
            let (p1, p2) = (String::from("x") + idx, String::from("y") + idx);
            let x1 = gs
                .get(&(&p1, &p2, OP::XOR))
                .or(gs.get(&(&p2, &p1, OP::XOR)))
                .unwrap();
            let x2 = m
                .unfinished
                .iter()
                .find(|g| g.op == OP::XOR && (g.in1 == x1.out || g.in2 == x1.out))
                .unwrap();
            swapped.push(&g.out);
            swapped.push(&x2.out);
        }

        //XOR connected to OR - swap with AND from same inputs
        if g.op == OP::XOR {
            if m.unfinished
                .iter()
                .find(|x| x.op == OP::OR && (x.in2 == g.out || x.in1 == g.out))
                .is_some()
            {
                let x = gs
                    .get(&(&g.in1, &g.in2, OP::AND))
                    .or(gs.get(&(&g.in2, &g.in1, OP::AND)))
                    .unwrap();
                swapped.push(&g.out);
                swapped.push(&x.out);
            }
        }
    }
    swapped.iter().sorted().join(",")
}
pub fn part_two(path: &str) -> Result<String, Box<dyn Error>> {
    let input = read_to_string(path).unwrap();
    let input = parse(input)?;
    let input = solve(&input);
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twenty_four_part_two() -> Result<(), Box<dyn Error>> {
        let output = part_two("./assets/day_twenty_four/sample.txt")?;
        assert_eq!(output, "aaa,aoc,bbb,ccc,eee,ooo,z24,z99");
        Ok(())
    }
}
