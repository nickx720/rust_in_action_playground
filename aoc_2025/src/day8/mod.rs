#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn find(&mut self, mut a: usize) -> usize {
        while self.parent[a] != a {
            self.parent[a] = self.parent[self.parent[a]];
            a = self.parent[a];
        }
        a
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    fn component_sizes(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for i in 0..self.parent.len() {
            if self.parent[i] == i {
                out.push(self.size[i]);
            }
        }
        out
    }
}

fn dist2(a: Point, b: Point) -> i128 {
    let dx = a.x as i128 - b.x as i128;
    let dy = a.y as i128 - b.y as i128;
    let dz = a.z as i128 - b.z as i128;
    dx * dx + dy * dy + dz * dz
}

fn parse_input(input: &str) -> Result<Vec<Point>, anyhow::Error> {
    let mut points = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(',');
        let x: i64 = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing x value"))?
            .parse()?;
        let y: i64 = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing y value"))?
            .parse()?;
        let z: i64 = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing z value"))?
            .parse()?;
        points.push(Point { x, y, z });
    }
    Ok(points)
}

fn solve(points: &[Point], attempts_limit: usize) -> usize {
    let n = points.len();
    let mut edges: Vec<(i128, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let d = dist2(points[i], points[j]);
            edges.push((d, i, j));
        }
    }
    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut dsu = Dsu::new(n);
    let mut attempts = 0usize;
    let mut k = 0usize;
    while attempts < attempts_limit && k < edges.len() {
        let (_, i, j) = edges[k];
        k += 1;
        attempts += 1;
        let _ = dsu.union(i, j);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

pub fn day8_partone(input: &str, attempts_limit: usize) -> Result<usize, anyhow::Error> {
    let points = parse_input(input)?;
    Ok(solve(&points, attempts_limit))
}

pub fn day8_partwo(input: &str) -> Result<usize, anyhow::Error> {
    let points = parse_input(input)?;
    let n = points.len();
    if n < 2 {
        return Ok(0);
    }

    let mut edges: Vec<(i128, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let d = dist2(points[i], points[j]);
            edges.push((d, i, j));
        }
    }
    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut dsu = Dsu::new(n);
    let mut components = n;
    let mut last_i = 0usize;
    let mut last_j = 0usize;

    for (_, i, j) in edges {
        if dsu.union(i, j) {
            components -= 1;
            last_i = i;
            last_j = j;
            if components == 1 {
                break;
            }
        }
    }

    let product = points[last_i].x as i128 * points[last_j].x as i128;
    let product = usize::try_from(product)
        .map_err(|_| anyhow::anyhow!("X product is out of range for usize"))?;
    Ok(product)
}
