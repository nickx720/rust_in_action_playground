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

pub fn day9_parttwo(input: &str) -> Result<usize, anyhow::Error> {
    let points = parse_input(input)?;
    let n = points.len();
    if n == 0 {
        return Ok(0);
    }

    let mut xs: Vec<isize> = points.iter().map(|(x, _)| *x).collect();
    let mut ys: Vec<isize> = points.iter().map(|(_, y)| *y).collect();

    let min_x = xs.iter().copied().min().unwrap_or(0);
    let max_x = xs.iter().copied().max().unwrap_or(0);
    let min_y = ys.iter().copied().min().unwrap_or(0);
    let max_y = ys.iter().copied().max().unwrap_or(0);

    xs.push(min_x.saturating_sub(1));
    xs.push(max_x.saturating_add(1));
    ys.push(min_y.saturating_sub(1));
    ys.push(max_y.saturating_add(1));

    let (x_segments, x_index) = build_segments(&mut xs);
    let (y_segments, y_index) = build_segments(&mut ys);

    let width = x_segments.len();
    let height = y_segments.len();

    let mut blocked = vec![vec![false; width]; height];
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        if x1 == x2 {
            let x_idx = *x_index
                .get(&x1)
                .ok_or_else(|| anyhow::anyhow!("Missing x coordinate in index"))?;
            let y_start = y1.min(y2);
            let y_end = y1.max(y2);
            let y_start_idx = *y_index
                .get(&y_start)
                .ok_or_else(|| anyhow::anyhow!("Missing y coordinate in index"))?;
            let y_end_idx = *y_index
                .get(&y_end)
                .ok_or_else(|| anyhow::anyhow!("Missing y coordinate in index"))?;
            for y_idx in y_start_idx..=y_end_idx {
                blocked[y_idx][x_idx] = true;
            }
        } else if y1 == y2 {
            let y_idx = *y_index
                .get(&y1)
                .ok_or_else(|| anyhow::anyhow!("Missing y coordinate in index"))?;
            let x_start = x1.min(x2);
            let x_end = x1.max(x2);
            let x_start_idx = *x_index
                .get(&x_start)
                .ok_or_else(|| anyhow::anyhow!("Missing x coordinate in index"))?;
            let x_end_idx = *x_index
                .get(&x_end)
                .ok_or_else(|| anyhow::anyhow!("Missing x coordinate in index"))?;
            for x_idx in x_start_idx..=x_end_idx {
                blocked[y_idx][x_idx] = true;
            }
        } else {
            return Err(anyhow::anyhow!("Non-axis-aligned edge detected"));
        }
    }

    let mut outside = vec![vec![false; width]; height];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0usize, 0usize));
    outside[0][0] = true;

    let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
    while let Some((cy, cx)) = queue.pop_front() {
        for (dx, dy) in dirs {
            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= width || ny >= height {
                continue;
            }
            if outside[ny][nx] || blocked[ny][nx] {
                continue;
            }
            outside[ny][nx] = true;
            queue.push_back((ny, nx));
        }
    }

    let mut allowed = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            if blocked[y][x] || !outside[y][x] {
                allowed[y][x] = true;
            }
        }
    }

    let mut pref = vec![vec![0i64; width + 1]; height + 1];
    for y in 1..=height {
        let seg_h = y_segments[y - 1].len;
        for x in 1..=width {
            let seg_w = x_segments[x - 1].len;
            let val = if allowed[y - 1][x - 1] {
                seg_w * seg_h
            } else {
                0
            };
            pref[y][x] = val + pref[y - 1][x] + pref[y][x - 1] - pref[y - 1][x - 1];
        }
    }

    let mut best: i64 = 0;
    for i in 0..n {
        let (x1, y1) = points[i];
        for j in i + 1..n {
            let (x2, y2) = points[j];
            let x_l = x1.min(x2);
            let x_r = x1.max(x2);
            let y_t = y1.min(y2);
            let y_b = y1.max(y2);
            let area = (x_r - x_l + 1) as i64 * (y_b - y_t + 1) as i64;
            if area <= best {
                continue;
            }
            let allowed_count = rect_sum(&pref, &x_index, &y_index, x_l, y_t, x_r, y_b)?;
            if allowed_count == area {
                best = area;
            }
        }
    }

    usize::try_from(best).map_err(|_| anyhow::anyhow!("Maximum area is out of range for usize"))
}

struct Segment {
    start: isize,
    end: isize,
    len: i64,
}

fn build_segments(coords: &mut Vec<isize>) -> (Vec<Segment>, std::collections::HashMap<isize, usize>) {
    coords.sort_unstable();
    coords.dedup();
    let mut segments = Vec::new();
    let mut index = std::collections::HashMap::new();
    for (i, &coord) in coords.iter().enumerate() {
        let seg_idx = segments.len();
        segments.push(Segment {
            start: coord,
            end: coord,
            len: 1,
        });
        index.insert(coord, seg_idx);
        if let Some(&next) = coords.get(i + 1) {
            if next - coord > 1 {
                segments.push(Segment {
                    start: coord + 1,
                    end: next - 1,
                    len: (next - coord - 1) as i64,
                });
            }
        }
    }
    (segments, index)
}

fn rect_sum(
    pref: &[Vec<i64>],
    x_index: &std::collections::HashMap<isize, usize>,
    y_index: &std::collections::HashMap<isize, usize>,
    x_l: isize,
    y_t: isize,
    x_r: isize,
    y_b: isize,
) -> Result<i64, anyhow::Error> {
    let x_l_idx = *x_index
        .get(&x_l)
        .ok_or_else(|| anyhow::anyhow!("Missing x coordinate in index"))?;
    let x_r_idx = *x_index
        .get(&x_r)
        .ok_or_else(|| anyhow::anyhow!("Missing x coordinate in index"))?;
    let y_t_idx = *y_index
        .get(&y_t)
        .ok_or_else(|| anyhow::anyhow!("Missing y coordinate in index"))?;
    let y_b_idx = *y_index
        .get(&y_b)
        .ok_or_else(|| anyhow::anyhow!("Missing y coordinate in index"))?;
    let i1 = y_t_idx + 1;
    let j1 = x_l_idx + 1;
    let i2 = y_b_idx + 1;
    let j2 = x_r_idx + 1;
    Ok(pref[i2][j2] - pref[i1 - 1][j2] - pref[i2][j1 - 1] + pref[i1 - 1][j1 - 1])
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
