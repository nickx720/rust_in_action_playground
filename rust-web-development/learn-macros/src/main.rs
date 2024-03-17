#![feature(trace_macros)]
#![feature(log_syntax)]
use crate::greeting::base_greeting_fn;
#[macro_use]
mod greeting;
fn main() {
    trace_macros!(true);
    greeting!("Nick", "Heya");
    greeting!(test "Heya");
    trace_macros!(false);
}
