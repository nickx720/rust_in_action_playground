mod parallelism;
use crate::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use parallelism::parallel_main;
// https://users.rust-lang.org/t/could-not-find-in-the-crate-root/61146/2
// todo convert parallelism to a lib not binary
fn main() {
    parallel_main();
}
