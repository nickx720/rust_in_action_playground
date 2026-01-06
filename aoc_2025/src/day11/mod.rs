pub fn day11_partone(input: &str) -> Result<usize, anyhow::Error> {
    let graph = parse_input(input)?;
    let mut memo: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    memo.insert("you".to_string(), 1);
    dfs_count("out", &graph, &mut memo)
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
