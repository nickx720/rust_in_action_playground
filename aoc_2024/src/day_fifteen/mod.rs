pub mod part_one;
pub mod part_two;

use std::fmt::Display;

type Map = Vec<Vec<Tile>>;
type WideMap = Vec<Vec<WideTile>>;

enum Tile {
    Empty,
    Robot,
    Box,
    Wall,
}

#[derive(Copy, Clone)]
enum WideTile {
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::Box,
            '.' => Tile::Empty,
            '@' => Tile::Robot,
            '#' => Tile::Wall,
            other => unreachable!("{other}"),
        }
    }
}

impl From<char> for WideTile {
    fn from(value: char) -> Self {
        match value {
            '[' => WideTile::BoxLeft,
            ']' => WideTile::BoxRight,
            '.' => WideTile::Empty,
            '@' => WideTile::Robot,
            '#' => WideTile::Wall,
            other => unreachable!("{other}"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Robot => write!(f, "@"),
            Tile::Box => write!(f, "O"),
            Tile::Wall => write!(f, "#"),
        }
    }
}

impl std::fmt::Display for WideTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WideTile::Empty => write!(f, "."),
            WideTile::Robot => write!(f, "@"),
            WideTile::BoxLeft => write!(f, "["),
            WideTile::BoxRight => write!(f, "]"),
            WideTile::Wall => write!(f, "#"),
        }
    }
}

// Helper function for debugging
#[allow(dead_code)]
fn print_grid<T: Display>(m: &[Vec<T>]) {
    for r in m {
        for c in r {
            print!("{c}");
        }
        println!();
    }
}

fn get_cell<T>(xy: (usize, usize), m: &[Vec<T>]) -> &T {
    &m[xy.1][xy.0]
}

fn char_to_wide_tiles(value: char) -> [WideTile; 2] {
    match value {
        'O' => [WideTile::BoxLeft, WideTile::BoxRight],
        '.' => [WideTile::Empty, WideTile::Empty],
        '@' => [WideTile::Robot, WideTile::Empty],
        '#' => [WideTile::Wall, WideTile::Wall],
        other => unreachable!("{other}"),
    }
}

/// (x, y)
fn robot_coord(m: &Map) -> (usize, usize) {
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, t) in r.iter().enumerate() {
            if matches!(t, Tile::Robot) {
                return (c_idx, r_idx);
            }
        }
    }
    unreachable!();
}

fn next_coords<const N: usize>(
    coords: [(usize, usize); N],
    mve: &Direction,
) -> [(usize, usize); N] {
    debug_assert!(matches!(mve, Direction::Up | Direction::Down));
    coords.map(|xy| next_coord(xy, mve))
}

fn next_empty_tile_coord(
    mut xy: (usize, usize),
    mve: &Direction,
    map: &Map,
) -> Option<(usize, usize)> {
    loop {
        xy = next_coord(xy, mve);
        match get_cell(xy, map) {
            Tile::Box => continue,
            Tile::Wall => return None,
            Tile::Empty => return Some(xy),
            Tile::Robot => unreachable!(),
        }
    }
}

fn next_coord(xy: (usize, usize), mve: &Direction) -> (usize, usize) {
    match mve {
        Direction::Up => (xy.0, xy.1 - 1),
        Direction::Down => (xy.0, xy.1 + 1),
        Direction::Left => (xy.0 - 1, xy.1),
        Direction::Right => (xy.0 + 1, xy.1),
    }
}
