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
