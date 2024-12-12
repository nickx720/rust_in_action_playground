pub mod part_one;
pub mod part_two;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DirectionEnum {
    North,
    East,
    West,
    South,
}

impl DirectionEnum {
    fn perpendicular(&self) -> Vec<DirectionEnum> {
        match self {
            DirectionEnum::North => vec![DirectionEnum::East, DirectionEnum::West],
            DirectionEnum::East => vec![DirectionEnum::North, DirectionEnum::South],
            DirectionEnum::South => vec![DirectionEnum::East, DirectionEnum::West],
            DirectionEnum::West => vec![DirectionEnum::North, DirectionEnum::South],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(i32, i32);

impl Position {
    fn north(&self) -> Position {
        let &Position(x, y) = self;
        Position(x, y - 1)
    }
    fn south(&self) -> Position {
        let &Position(x, y) = self;
        Position(x, y + 1)
    }
    fn east(&self) -> Position {
        let &Position(x, y) = self;
        Position(x + 1, y)
    }
    fn west(&self) -> Position {
        let &Position(x, y) = self;
        Position(x - 1, y)
    }
}

pub struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn ylen(&self) -> usize {
        self.0.len()
    }
    fn xlen(&self) -> usize {
        if self.ylen() > 0 {
            return self.0[0].len();
        }
        0
    }
    fn is_valid(&self, position: Position) -> bool {
        position.0 >= 0
            && position.1 >= 0
            && position.0 < self.xlen() as i32
            && position.1 < self.ylen() as i32
    }
    fn at(&self, position: Position) -> u8 {
        self.0[position.1 as usize][position.0 as usize]
    }
    fn get(&self, position: Position, direction: DirectionEnum) -> Option<Position> {
        match direction {
            DirectionEnum::North => Some(position.north()),
            DirectionEnum::East => Some(position.east()),
            DirectionEnum::South => Some(position.south()),
            DirectionEnum::West => Some(position.west()),
        }
        .filter(|&n| self.is_valid(n))
    }
    fn neighbors(&self, position: Position) -> Vec<Position> {
        let mut result = Vec::new();
        for d in [
            DirectionEnum::North,
            DirectionEnum::East,
            DirectionEnum::South,
            DirectionEnum::West,
        ] {
            if let Some(neighbour) = self.get(position, d) {
                if self.at(neighbour) == self.at(position) {
                    result.push(neighbour);
                }
            }
        }
        result
    }

    fn outer_count(&self, position: Position) -> usize {
        let mut count = 0;
        for d in [
            DirectionEnum::North,
            DirectionEnum::East,
            DirectionEnum::South,
            DirectionEnum::West,
        ] {
            if let Some(n) = self.get(position, d) {
                if self.at(n) != self.at(position) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
        count
    }
}
