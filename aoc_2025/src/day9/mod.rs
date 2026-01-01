pub fn day9_partone(input: &str) -> Result<usize, anyhow::Error> {
    let points = parse_input(input)?;
    let n = points.len();
    let mut best: isize = 0;

    for i in 0..n {
        let (x1, y1) = points[i];
        for j in i + 1..n {
            let (x2, y2) = points[j];
            let dx = (x1 - x2).abs();
            let dy = (y1 - y2).abs();
            let area = (dx + 1) * (dy + 1);
            if area > best {
                best = area;
            }
        }
    }

    let best = usize::try_from(best)
        .map_err(|_| anyhow::anyhow!("Maximum area is out of range for usize"))?;
    Ok(best)
}

fn parse_input(input: &str) -> Result<Vec<(isize, isize)>, anyhow::Error> {
    let mut points = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(',');
        let x: isize = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing x value"))?
            .parse()?;
        let y: isize = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing y value"))?
            .parse()?;
        points.push((x, y));
    }
    Ok(points)
}
