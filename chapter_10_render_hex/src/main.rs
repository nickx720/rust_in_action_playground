// https://users.rust-lang.org/t/could-not-find-in-the-crate-root/61146/2
// todo convert parallelism to a lib not binary
#[macro_use]
extern crate crossbeam;

mod channels_intro;
mod parallelism;
mod threadpool;
use channels_intro::channels_intro_main;
use parallelism::parallel_main;
use threadpool::threadpool_main;
fn main() {
    //    parallel_main();
    // channels_intro_main();
    threadpool_main();
}
