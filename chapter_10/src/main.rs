mod loop_waiting;
mod suspend_thread;
mod thread_example;
use thread_example::{thread_example_main, thread_two_threads_example};
fn main() {
    //    thread_example_main();
    //    thread_two_threads_example();
    //    suspend_thread::suspend_thread_main();
    loop_waiting::loop_waiting_main();
}
