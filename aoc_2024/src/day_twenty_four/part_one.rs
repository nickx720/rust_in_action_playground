use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::read_to_string,
};

use super::{parse_input, Gate, GateOp};

fn wires_to_decimal(wires: &HashMap<String, Option<bool>>, prefix: &str) -> u64 {
    let mut decimal = 0;
    let num_prefixed_wires = wires.keys().filter(|x| x.starts_with(prefix)).count();
    for i in 0..num_prefixed_wires {
        let wire_name = format!("{}{:02}", prefix, i);
        let wire_value = wires.get(&wire_name).unwrap().unwrap();
        if wire_value {
            decimal += 2_u64.pow(i as u32);
        }
    }
    decimal
}

fn compute(wires: &mut HashMap<String, Option<bool>>, gates: &Vec<Gate>) -> Result<(), String> {
    let mut queue: VecDeque<Gate> = VecDeque::new();
    for gate in gates {
        queue.push_back(gate.clone());
    }

    let mut fails = 0;
    while let Some(g) = queue.pop_front() {
        let val_a = wires.get(&g.inputs.0).unwrap();
        let val_b = wires.get(&g.inputs.1).unwrap();
        if val_a.is_none() || val_b.is_none() {
            queue.push_back(g);
            fails += 1;
            if fails >= queue.len() {
                return Err(String::from("Couldn't compute wire values"));
            }
            continue;
        }
        fails = 0;

        let val_out = g.op.apply(val_a.unwrap(), val_b.unwrap());
        wires.insert(g.output, Some(val_out));
    }
    Ok(())
}

pub fn part_one(path: &str) -> Result<u64, Box<dyn Error>> {
    let input = read_to_string(path).unwrap();
    let (mut wires, gates) = parse_input(&input);
    compute(&mut wires, &gates).unwrap();
    let decimal = wires_to_decimal(&wires, "z");
    Ok(decimal)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twenty_four_part_one() -> Result<(), Box<dyn Error>> {
        let output = part_one("./assets/day_twenty_four/sample.txt")?;
        assert_eq!(output, 2024);
        Ok(())
    }
}
