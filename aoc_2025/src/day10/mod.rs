use crate::simplex::*;

pub fn day10_partone(input: &str) -> Result<usize, anyhow::Error> {
    let mut total: usize = 0;
    for (line_idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (diagram, buttons) =
            parse_line(line).map_err(|err| anyhow::anyhow!("Line {}: {}", line_idx + 1, err))?;
        let best = solve_machine(&diagram, &buttons)?;
        total = total
            .checked_add(best)
            .ok_or_else(|| anyhow::anyhow!("Total exceeds usize range"))?;
    }
    Ok(total)
}

pub fn day10_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let mut total: usize = 0;
    for (line_idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (diagram, buttons) =
            parse_line(line).map_err(|err| anyhow::anyhow!("Line {}: {}", line_idx + 1, err))?;
        let targets = parse_targets(line, diagram.len())
            .map_err(|err| anyhow::anyhow!("Line {}: {}", line_idx + 1, err))?;
        validate_button_indices(&buttons, diagram.len())
            .map_err(|err| anyhow::anyhow!("Line {}: {}", line_idx + 1, err))?;
        let machine = Machine {
            buttons,
            jolts: targets,
        };
        let best = fewest_presses(machine)?;
        total = total
            .checked_add(best)
            .ok_or_else(|| anyhow::anyhow!("Total exceeds usize range"))?;
    }
    Ok(total)
}

fn parse_line(line: &str) -> Result<(String, Vec<Vec<usize>>), anyhow::Error> {
    let start = line
        .find('[')
        .ok_or_else(|| anyhow::anyhow!("Missing '[' in line"))?;
    let end = line[start + 1..]
        .find(']')
        .ok_or_else(|| anyhow::anyhow!("Missing ']' in line"))?
        + start
        + 1;
    let diagram = line[start + 1..end].trim().to_string();
    if diagram.is_empty() {
        return Err(anyhow::anyhow!("Empty diagram"));
    }

    let mut buttons = Vec::new();
    let mut idx = end + 1;
    while let Some(rel_start) = line[idx..].find('(') {
        let open = idx + rel_start;
        let close = line[open + 1..]
            .find(')')
            .ok_or_else(|| anyhow::anyhow!("Missing ')' for button list"))?
            + open
            + 1;
        let content = line[open + 1..close].trim();
        let mut indices = Vec::new();
        if !content.is_empty() {
            for part in content.split(',') {
                let value = part
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| anyhow::anyhow!("Invalid index '{}'", part.trim()))?;
                indices.push(value);
            }
        }
        buttons.push(indices);
        idx = close + 1;
    }

    Ok((diagram, buttons))
}

fn parse_targets(line: &str, n: usize) -> Result<Vec<usize>, anyhow::Error> {
    let start = line
        .find('{')
        .ok_or_else(|| anyhow::anyhow!("Missing '{{' for target list"))?;
    let end = line[start + 1..]
        .find('}')
        .ok_or_else(|| anyhow::anyhow!("Missing '}}' for target list"))?
        + start
        + 1;
    let content = line[start + 1..end].trim();
    if content.is_empty() {
        return Err(anyhow::anyhow!("Empty target list"));
    }
    let mut values = Vec::new();
    for part in content.split(',') {
        let text = part.trim();
        if text.is_empty() {
            return Err(anyhow::anyhow!("Empty target entry"));
        }
        let value = text
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("Invalid target '{}'", text))?;
        values.push(value);
    }
    if values.len() != n {
        return Err(anyhow::anyhow!(
            "Target count {} does not match diagram length {}",
            values.len(),
            n
        ));
    }
    Ok(values)
}

fn validate_button_indices(buttons: &[Vec<usize>], n: usize) -> Result<(), anyhow::Error> {
    for (j, indices) in buttons.iter().enumerate() {
        for &idx in indices {
            if idx >= n {
                return Err(anyhow::anyhow!("Button {} index {} out of range", j, idx));
            }
        }
    }
    Ok(())
}

fn solve_machine(diagram: &str, buttons: &[Vec<usize>]) -> Result<usize, anyhow::Error> {
    let n = diagram.len();
    let m = buttons.len();
    if n > 128 {
        return Err(anyhow::anyhow!("Diagram length {} exceeds 128", n));
    }
    if m > 128 {
        return Err(anyhow::anyhow!("Button count {} exceeds 128", m));
    }

    let target = mask_from_diagram(diagram)?;
    let mut button_masks = Vec::with_capacity(m);
    for indices in buttons {
        let mask = mask_from_indices(indices, n)?;
        button_masks.push(mask);
    }

    let mut basis_mask = vec![0u128; n];
    let mut basis_sol = vec![0u128; n];

    for (j, &mask) in button_masks.iter().enumerate() {
        let mut v = mask;
        let mut s = 1u128 << j;
        while v != 0 {
            let p = msb_index(v);
            if basis_mask[p] == 0 {
                basis_mask[p] = v;
                basis_sol[p] = s;
                break;
            } else {
                v ^= basis_mask[p];
                s ^= basis_sol[p];
            }
        }
    }

    let mut v = target;
    let mut x0: u128 = 0;
    while v != 0 {
        let p = msb_index(v);
        if basis_mask[p] == 0 {
            return Err(anyhow::anyhow!("Unsatisfiable target"));
        }
        v ^= basis_mask[p];
        x0 ^= basis_sol[p];
    }

    let mut null: Vec<u128> = Vec::new();
    for (j, &mask) in button_masks.iter().enumerate() {
        let mut v = mask;
        let mut s = 1u128 << j;
        while v != 0 {
            let p = msb_index(v);
            if basis_mask[p] == 0 {
                break;
            }
            v ^= basis_mask[p];
            s ^= basis_sol[p];
        }
        if v == 0 && s != 0 {
            null.push(s);
        }
    }

    let null = reduce_nullspace(null, m);
    let mut best = popcount(x0);
    let k = null.len();
    if k >= 128 {
        return Err(anyhow::anyhow!("Nullspace size {} exceeds 127", k));
    }
    if k > 0 {
        let limit = 1u128 << k;
        for t in 1..limit {
            let mut x = x0;
            for i in 0..k {
                if (t >> i) & 1 == 1 {
                    x ^= null[i];
                }
            }
            let weight = popcount(x);
            if weight < best {
                best = weight;
            }
        }
    }

    Ok(best)
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    jolts: Vec<usize>,
}

fn branch_and_bound(root: LPBuilder, n: usize) -> Option<i64> {
    let mut best: Option<i64> = None;
    let mut stack = vec![root];
    while let Some(b) = stack.pop() {
        let mut lp = b.clone().build();
        let Some(obj) = lp.minimize() else {
            continue;
        };
        let node_lb = obj.ceil();
        if let Some(best_val) = best {
            if node_lb >= best_val.into() {
                continue;
            }
        }
        let x = lp.solution_x();
        if let Some((k, xk)) = x.iter().enumerate().find(|(_, v)| !v.is_integer()) {
            let lo = xk.floor().to_integer();
            let hi = xk.ceil().to_integer();
            let mut b_le = b.clone();
            let mut v = vec![0; n];
            v[k] = 1;
            b_le.add_constraint(v.clone(), LPOp::Lte, lo);
            let mut b_ge = b;
            b_ge.add_constraint(v, LPOp::Gte, hi);
            stack.push(b_le);
            stack.push(b_ge);
        } else {
            let obj_i = obj.to_integer();
            best = Some(best.map_or(obj_i, |cur| cur.min(obj_i)));
        }
    }
    best
}

fn fewest_presses(m: Machine) -> Result<usize, anyhow::Error> {
    let mut builder = LPBuilder::new();
    let n = m.buttons.len();
    for (counter, j) in m.jolts.iter().copied().enumerate() {
        let vars = m
            .buttons
            .iter()
            .map(|button| button.contains(&counter) as i64)
            .collect::<Vec<_>>();
        builder.add_constraint(vars, LPOp::Eq, j as i64);
    }
    builder.add_objective(vec![1; n]);
    branch_and_bound(builder, n)
        .map(|value| value as usize)
        .ok_or_else(|| anyhow::anyhow!("No integer solution found"))
}


fn reduce_nullspace(null: Vec<u128>, m: usize) -> Vec<u128> {
    let mut basis = vec![0u128; m];
    for mut v in null {
        while v != 0 {
            let p = msb_index(v);
            if basis[p] == 0 {
                basis[p] = v;
                break;
            }
            v ^= basis[p];
        }
    }
    basis.into_iter().filter(|v| *v != 0).collect()
}

fn mask_from_diagram(diagram: &str) -> Result<u128, anyhow::Error> {
    let mut mask = 0u128;
    for (i, ch) in diagram.chars().enumerate() {
        match ch {
            '#' => mask |= 1u128 << i,
            '.' => {}
            _ => return Err(anyhow::anyhow!("Unexpected diagram char '{}'", ch)),
        }
    }
    Ok(mask)
}

fn mask_from_indices(indices: &[usize], n: usize) -> Result<u128, anyhow::Error> {
    let mut mask = 0u128;
    for &idx in indices {
        if idx >= n {
            return Err(anyhow::anyhow!("Button index {} out of range", idx));
        }
        mask |= 1u128 << idx;
    }
    Ok(mask)
}

fn msb_index(v: u128) -> usize {
    127 - v.leading_zeros() as usize
}

fn popcount(v: u128) -> usize {
    v.count_ones() as usize
}
