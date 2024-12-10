use std::collections::HashSet;

pub mod part_one;
pub mod part_two;
pub fn dfs(
    pos_y: usize,
    pos_x: usize,
    grid: &Vec<Vec<usize>>,
    hash_set: &mut HashSet<(usize, usize)>,
    part_one: bool,
) -> usize {
    if part_one {
        if hash_set.contains(&(pos_y, pos_x)) {
            return 0;
        }
        hash_set.insert((pos_y, pos_x));
    }
    if grid[pos_y][pos_x] == 9 {
        return 1;
    }
    let mut result = 0;
    for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = pos_y as isize + dir.0;
        let ny = pos_x as isize + dir.1;
        if nx >= 0
            && ny >= 0
            && (nx as usize) < grid[0].len()
            && (ny as usize) < grid.len()
            && grid[nx as usize][ny as usize] == 1 + grid[pos_y][pos_x]
        {
            result += dfs(nx as usize, ny as usize, grid, hash_set, part_one);
        }
    }
    result
}
