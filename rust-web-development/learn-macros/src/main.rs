//use crate::greeting::base_greeting_fn;

use crate::calculate::{FirstName, LastName};
//#[macro_use]
//mod greeting;
#[macro_use]
mod calculate;
struct Age {
    value: String,
}
fn main() {
    generate_get_value!(FirstName, String);
    generate_get_value!(Age, String);
    let first = FirstName::new("John");
    let value_of = first.unwrap().get_value().clone();
    let age = Age::new("Cortanna");
    let age_va = age.unwrap().get_value().clone();

    dbg!(value_of);
    dbg!(age_va);
    //    trace_macros!(true);
    //    greeting!("Nick", "Heya");
    //    greeting!(test "Heya");
    //    trace_macros!(false);
}
