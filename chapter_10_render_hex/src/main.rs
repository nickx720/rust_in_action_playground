// https://users.rust-lang.org/t/could-not-find-in-the-crate-root/61146/2
// todo convert parallelism to a lib not binary
mod parallelism;
use parallelism::parallel_main;
fn main() {
    parallel_main();
}
