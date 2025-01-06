use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

use crate::day_twenty_four::parse_input;

use super::{Gate, GateOp};

fn build_input_map(wires: &Vec<String>, gates: &Vec<Gate>) -> HashMap<String, Vec<Gate>> {
    let mut map = HashMap::new();
    for wire in wires {
        let mut gates_in = Vec::new();
        for gate in gates {
            if &gate.inputs.0 == wire || &gate.inputs.1 == wire {
                gates_in.push(gate.clone());
            }
        }
        map.insert(wire.to_string(), gates_in);
    }
    map
}

fn find_nth_bit_adder(n: usize, input_map: &HashMap<String, Vec<Gate>>) {
    let x_name = format!("x{:02}", n);
    let y_name = format!("y{:02}", n);

    let mut gates: HashSet<Gate> = HashSet::new();
    for gate in input_map.get(&x_name).unwrap() {
        gates.insert(gate.clone());
    }
    for gate in input_map.get(&y_name).unwrap() {
        gates.insert(gate.clone());
    }
    let mut next_gates: HashSet<Gate> = HashSet::new();
    for gate in &gates {
        if !gate.output.starts_with("z") {
            for next_gate in input_map.get(&gate.output).unwrap() {
                next_gates.insert(next_gate.clone());
            }
        }
    }
    gates.extend(next_gates);

    for gate in gates {
        gate.print();
        println!(" is part of bit {}", n);
    }
}

fn is_xyz(wire: &str) -> bool {
    wire.starts_with("x") || wire.starts_with("y") || wire.starts_with("z")
}

fn or_gates_no_xyz(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::OR {
            continue;
        }
        if is_xyz(&gate.inputs.0) {
            bad_wires.push(gate.inputs.0.clone());
        }
        if is_xyz(&gate.inputs.1) {
            bad_wires.push(gate.inputs.1.clone());
        }
        if is_xyz(&gate.output) {
            if gate.output != "z45" {
                bad_wires.push(gate.output.clone());
            }
        }
    }
    bad_wires
}

fn and_gates_no_xyz_output(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::AND {
            continue;
        }
        if is_xyz(&gate.output) {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn and_xor_gates_both_xyz_or_none(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if ![GateOp::AND, GateOp::XOR].contains(&gate.op) {
            continue;
        }
        if (is_xyz(&gate.inputs.0) && !is_xyz(&gate.inputs.1))
            || (!is_xyz(&gate.inputs.0) && is_xyz(&gate.inputs.1))
        {
            bad_wires.push(gate.inputs.0.clone());
            bad_wires.push(gate.inputs.1.clone());
        }
    }
    bad_wires
}

fn and_output_is_or_input(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        // pjf is my bit-0 input carry, this has to be changed
        // for other inputs
        if gate.op != GateOp::AND || gate.output == "pjf" {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 1 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if next_gates[0].op != GateOp::OR {
            bad_wires.push(gate.output.clone());
            continue;
        }
    }
    bad_wires
}

fn or_output_goes_in_one_and_one_xor(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::OR || gate.output == "z45" {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 2 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if !((next_gates[0].op == GateOp::AND && next_gates[1].op == GateOp::XOR)
            || (next_gates[0].op == GateOp::XOR && next_gates[1].op == GateOp::AND))
        {
            bad_wires.push(gate.output.clone());
            continue;
        }
    }
    bad_wires
}

fn xor_output_non_z_goes_in_one_and_one_xor(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::XOR || gate.output.starts_with("z") {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 2 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if !((next_gates[0].op == GateOp::AND && next_gates[1].op == GateOp::XOR)
            || (next_gates[0].op == GateOp::XOR && next_gates[1].op == GateOp::AND))
        {
            bad_wires.push(gate.output.clone());
            continue;
        }
    }
    bad_wires
}

fn xor_with_non_xy_in_has_z_out(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::XOR || is_xyz(&gate.inputs.0) || is_xyz(&gate.inputs.1) {
            continue;
        }

        if !gate.output.starts_with("z") {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

pub fn part_two(path: &str) -> Result<String, Box<dyn Error>> {
    let input = read_to_string(path).unwrap();
    let (wires, gates) = parse_input(&input);
    let input_map = build_input_map(&wires.keys().cloned().collect(), &gates);
    for n in 0..45 {
        find_nth_bit_adder(n, &input_map);
        println!("");
    }

    let mut bad_wires: HashSet<String> = HashSet::new();
    let bad = or_gates_no_xyz(&gates);
    println!("CHECK: OR gates can't have xyz wires in or out: {:?}", bad);
    bad_wires.extend(bad);

    let bad = and_gates_no_xyz_output(&gates);
    println!("CHECK: AND gates can't have xyz outputs: {:?}", bad);
    bad_wires.extend(bad);

    let bad = and_xor_gates_both_xyz_or_none(&gates);
    println!(
        "CHECK: AND/XOR gate inputs are both or neither xyz: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = and_output_is_or_input(&gates, &input_map);
    println!("CHECK: AND outputs are followed by a single OR: {:?}", bad);
    bad_wires.extend(bad);

    let bad = or_output_goes_in_one_and_one_xor(&gates, &input_map);
    println!(
        "CHECK: OR outputs go in exactly one AND & one XOR: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = xor_output_non_z_goes_in_one_and_one_xor(&gates, &input_map);
    println!(
        "CHECK: Non-z XOR outputs go in exactly one AND & one XOR: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = xor_with_non_xy_in_has_z_out(&gates);
    println!("CHECK: XOR with non-xy inputs has z output: {:?}", bad);
    bad_wires.extend(bad);

    let mut sorted: Vec<String> = bad_wires.into_iter().collect();
    sorted.sort();
    println!("");
    Ok(sorted.join(","))
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
