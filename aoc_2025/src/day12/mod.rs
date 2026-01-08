pub fn day12_partone(input: &str) -> Result<usize, anyhow::Error> {
    let (shapes, cases) = parse_input(input)?;
    let mut count: usize = 0;
    for (case_idx, case) in cases.iter().enumerate() {
        if is_trivially_possible(case) {
            count = count
                .checked_add(1)
                .ok_or_else(|| anyhow::anyhow!("Count exceeds usize range"))?;
        } else if !is_trivially_impossible(&shapes, case)? {
            return Err(anyhow::anyhow!(
                "Case {} is actually non-trivial",
                case_idx + 1
            ));
        }
    }
    Ok(count)
}

struct Case {
    region: (u32, u32),
    shape_counts: Vec<u32>,
}

fn is_trivially_possible(case: &Case) -> bool {
    // every shape fits into a 3x3 box
    let num_shapes: u32 = case.shape_counts.iter().sum();
    (case.region.0 / 3) * (case.region.1 / 3) >= num_shapes
}

fn is_trivially_impossible(shapes: &[u32], case: &Case) -> Result<bool, anyhow::Error> {
    if case.shape_counts.len() > shapes.len() {
        return Err(anyhow::anyhow!(
            "Case expects {} shapes but only {} defined",
            case.shape_counts.len(),
            shapes.len()
        ));
    }
    let total_shape_area: u32 = case
        .shape_counts
        .iter()
        .enumerate()
        .map(|(i, &num)| shapes[i] * num)
        .sum();
    Ok(case.region.0 * case.region.1 < total_shape_area)
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<Case>), anyhow::Error> {
    let mut shape_sizes = Vec::new();
    let mut shape_lines: usize = 0;
    let mut shape_size: usize = 0;
    let mut cases = Vec::new();

    for (line_idx, line) in input.lines().enumerate() {
        let raw_line = line.trim();
        if raw_line.is_empty() {
            continue;
        }
        if raw_line.contains('x') {
            cases.push(
                parse_case(raw_line)
                    .map_err(|err| anyhow::anyhow!("Line {}: {}", line_idx + 1, err))?,
            );
        } else if raw_line.contains('.') || raw_line.contains('#') {
            // ignore the shape, just count the squares
            shape_size += raw_line.chars().filter(|&ch| ch == '#').count();
            shape_lines += 1;
            if shape_lines == 3 {
                shape_sizes.push(
                    u32::try_from(shape_size)
                        .map_err(|_| anyhow::anyhow!("Line {}: shape too large", line_idx + 1))?,
                );
                shape_size = 0;
                shape_lines = 0;
            }
        }
    }

    if shape_lines != 0 {
        return Err(anyhow::anyhow!("Trailing incomplete shape"));
    }

    Ok((shape_sizes, cases))
}

fn parse_case(line: &str) -> Result<Case, anyhow::Error> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty case line"));
    }
    let header = parts[0];
    let mut dims = header.trim_end_matches(':').split('x');
    let width = dims
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing width"))?
        .parse::<u32>()
        .map_err(|_| anyhow::anyhow!("Invalid width"))?;
    let height = dims
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing height"))?
        .parse::<u32>()
        .map_err(|_| anyhow::anyhow!("Invalid height"))?;
    if dims.next().is_some() {
        return Err(anyhow::anyhow!("Too many dimensions"));
    }
    if parts.len() == 1 {
        return Err(anyhow::anyhow!("Missing shape counts"));
    }

    let mut shape_counts = Vec::with_capacity(parts.len() - 1);
    for raw in &parts[1..] {
        let value = raw
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Invalid shape count '{}'", raw))?;
        shape_counts.push(value);
    }

    Ok(Case {
        region: (width, height),
        shape_counts,
    })
}
