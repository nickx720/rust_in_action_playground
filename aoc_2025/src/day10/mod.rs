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
        let (target, buttons) = parse_line_with_targets(line)
            .map_err(|err| anyhow::anyhow!("Line {}: {}", line_idx + 1, err))?;
        let best = solve_machine_greedy_lookahead(&target, &buttons, line_idx)?;
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

fn parse_line_with_targets(line: &str) -> Result<(Vec<usize>, Vec<Vec<usize>>), anyhow::Error> {
    let (diagram, buttons) = parse_line(line)?;
    let n = diagram.len();
    let target = if let Some(start) = line.find('{') {
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
        values
    } else {
        let mut values = Vec::with_capacity(n);
        for ch in diagram.chars() {
            match ch {
                '#' => values.push(1),
                '.' => values.push(0),
                _ => return Err(anyhow::anyhow!("Unexpected diagram char '{}'", ch)),
            }
        }
        values
    };

    for (j, indices) in buttons.iter().enumerate() {
        if indices.is_empty() {
            continue;
        }
        for &idx in indices {
            if idx >= n {
                return Err(anyhow::anyhow!("Button {} index {} out of range", j, idx));
            }
        }
    }

    Ok((target, buttons))
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

fn solve_machine_greedy_lookahead(
    target: &[usize],
    buttons: &[Vec<usize>],
    line_idx: usize,
) -> Result<usize, anyhow::Error> {
    if target.is_empty() {
        return Ok(0);
    }

    greedy_construct_lookahead(target, buttons, line_idx)?
        .ok_or_else(|| anyhow::anyhow!("Greedy lookahead failed"))
}

fn greedy_construct_lookahead(
    target: &[usize],
    buttons: &[Vec<usize>],
    _line_idx: usize,
) -> Result<Option<usize>, anyhow::Error> {
    let mut need = target.to_vec();
    let mut remaining: u128 = need
        .iter()
        .try_fold(0u128, |acc, &v| acc.checked_add(v as u128))
        .ok_or_else(|| anyhow::anyhow!("Total target exceeds u128 range"))?;
    let mut total_presses: u128 = 0;

    while remaining > 0 {
        let mut best_j: Option<usize> = None;
        let mut best_score: i128 = i128::MIN;
        let mut best_blocked: usize = usize::MAX;
        let mut best_impact: i128 = i128::MIN;
        let mut best_helpful: i128 = i128::MIN;
        for (j, indices) in buttons.iter().enumerate() {
            if indices.is_empty() {
                continue;
            }
            let mut tight = usize::MAX;
            let mut impact: i128 = 0;
            let mut feasible = true;
            let mut zeros_after: Vec<usize> = Vec::new();
            for &idx in indices {
                let need_i = need[idx];
                if need_i == 0 {
                    feasible = false;
                    break;
                }
                if need_i < tight {
                    tight = need_i;
                }
                impact += need_i as i128;
                if need_i == 1 {
                    zeros_after.push(idx);
                }
            }
            if !feasible {
                continue;
            }

            let helpful = indices.len() as i128;
            let tight = tight as i128;
            let blocked = if zeros_after.is_empty() {
                0usize
            } else {
                count_blocked_buttons(buttons, &zeros_after)
            };
            let score = 1000 * helpful + 2 * impact - 50 * tight - 200 * (blocked as i128);
            if score > best_score
                || (score == best_score && blocked < best_blocked)
                || (score == best_score && blocked == best_blocked && impact > best_impact)
                || (score == best_score
                    && blocked == best_blocked
                    && impact == best_impact
                    && helpful > best_helpful)
            {
                best_score = score;
                best_j = Some(j);
                best_blocked = blocked;
                best_impact = impact;
                best_helpful = helpful;
            }
        }

        let j = match best_j {
            Some(idx) => idx,
            None => return Ok(None),
        };

        total_presses = total_presses
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("Press count exceeds u128 range"))?;
        let touched = &buttons[j];
        for &idx in touched {
            need[idx] -= 1;
        }
        remaining = remaining
            .checked_sub(touched.len() as u128)
            .ok_or_else(|| anyhow::anyhow!("Remaining count underflow"))?;
    }

    usize::try_from(total_presses)
        .map(Some)
        .map_err(|_| anyhow::anyhow!("Press count exceeds usize range"))
}

fn count_blocked_buttons(buttons: &[Vec<usize>], zeros_after: &[usize]) -> usize {
    let mut blocked = 0;
    for indices in buttons {
        if indices.iter().any(|idx| zeros_after.contains(idx)) {
            blocked += 1;
        }
    }
    blocked
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
