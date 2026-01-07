pub fn day11_partone(input: &str) -> Result<usize, anyhow::Error> {
    let graph = parse_input(input)?;
    let mut memo: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    memo.insert("you".to_string(), 1);
    dfs_count("out", &graph, &mut memo)
}

pub fn day11_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let graph = parse_input(input)?;
    let mut memo: std::collections::HashMap<String, (usize, usize, usize, usize)> =
        std::collections::HashMap::new();
    memo.insert("svr".to_string(), (1, 0, 0, 0));
    let (_a, _b, _c, d) = dfs_count_quad("out", &graph, &mut memo)?;
    Ok(d)
}

fn parse_input(
    input: &str,
) -> Result<std::collections::HashMap<String, Vec<String>>, anyhow::Error> {
    let mut graph: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for (line_idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, ':');
        let node = parts
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Line {}: missing node", line_idx + 1))?;
        let outputs = parts
            .next()
            .map(str::trim)
            .ok_or_else(|| anyhow::anyhow!("Line {}: missing ':' separator", line_idx + 1))?;

        if outputs.is_empty() {
            continue;
        }

        for raw in outputs.split_whitespace() {
            let output = raw.trim_end_matches(',');
            if output.is_empty() {
                continue;
            }
            graph
                .entry(output.to_string())
                .or_default()
                .push(node.to_string());
        }
    }

    Ok(graph)
}

fn dfs_count(
    node: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    memo: &mut std::collections::HashMap<String, usize>,
) -> Result<usize, anyhow::Error> {
    if let Some(&cached) = memo.get(node) {
        return Ok(cached);
    }

    let parents = match graph.get(node) {
        Some(list) => list,
        None => return Ok(0),
    };

    let mut total: usize = 0;
    for parent in parents {
        let count = dfs_count(parent, graph, memo)?;
        total = total
            .checked_add(count)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
    }

    memo.insert(node.to_string(), total);
    Ok(total)
}

fn dfs_count_quad(
    node: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    memo: &mut std::collections::HashMap<String, (usize, usize, usize, usize)>,
) -> Result<(usize, usize, usize, usize), anyhow::Error> {
    if let Some(&cached) = memo.get(node) {
        return Ok(cached);
    }

    let parents = match graph.get(node) {
        Some(list) => list,
        None => return Ok((0, 0, 0, 0)),
    };

    let mut total_a: usize = 0;
    let mut total_b: usize = 0;
    let mut total_c: usize = 0;
    let mut total_d: usize = 0;

    for parent in parents {
        let (a, b, c, d) = dfs_count_quad(parent, graph, memo)?;
        total_a = total_a
            .checked_add(a)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
        total_b = total_b
            .checked_add(b)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
        total_c = total_c
            .checked_add(c)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
        total_d = total_d
            .checked_add(d)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
    }

    if node == "dac" {
        total_d = total_d
            .checked_add(total_b)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
        total_c = total_c
            .checked_add(total_a)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
    }

    if node == "fft" {
        total_d = total_d
            .checked_add(total_c)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
        total_b = total_b
            .checked_add(total_a)
            .ok_or_else(|| anyhow::anyhow!("Path count exceeds usize range"))?;
    }

    let totals = (total_a, total_b, total_c, total_d);
    memo.insert(node.to_string(), totals);
    Ok(totals)
}
