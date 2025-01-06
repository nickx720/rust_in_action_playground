use std::collections::HashMap;

pub mod part_one;
pub mod part_two;
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum GateOp {
    AND,
    OR,
    XOR,
}

impl GateOp {
    fn from_str(str: &str) -> GateOp {
        match str {
            "AND" => GateOp::AND,
            "OR" => GateOp::OR,
            "XOR" => GateOp::XOR,
            _ => panic!(),
        }
    }

    fn apply(&self, a: bool, b: bool) -> bool {
        match &self {
            GateOp::AND => a && b,
            GateOp::OR => a || b,
            GateOp::XOR => a ^ b,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Gate {
    inputs: (String, String),
    output: String,
    op: GateOp,
}

impl Gate {
    fn print(&self) {
        print!(
            "{} {:?} {} -> {}",
            self.inputs.0, self.op, self.inputs.1, self.output
        );
    }
}

fn bool_from_char(c: char) -> bool {
    match c {
        '0' => false,
        '1' => true,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> (HashMap<String, Option<bool>>, Vec<Gate>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut wires: HashMap<String, Option<bool>> = HashMap::new();
    let mut gates = Vec::new();

    let mut i = 0;
    while lines[i].trim() != "" {
        wires.insert(
            lines[i][..3].to_string(),
            Some(bool_from_char(lines[i].chars().nth(5).unwrap())),
        );
        i += 1;
    }

    for line in lines.iter().skip(i + 1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let input_a = parts[0].to_string();
        let op = GateOp::from_str(&parts[1]);
        let input_b = parts[2].to_string();
        let output = parts[4].to_string();

        gates.push(Gate {
            inputs: (input_a.clone(), input_b.clone()),
            op,
            output: output.clone(),
        });

        for wire in [input_a, input_b, output] {
            if !wires.contains_key(&wire) {
                wires.insert(wire.to_string(), None);
            }
        }
    }

    (wires, gates)
}
