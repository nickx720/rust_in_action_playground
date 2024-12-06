pub mod part_one;
pub mod part_two;

pub fn turn(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}
