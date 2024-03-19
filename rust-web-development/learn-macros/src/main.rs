use crate::calculate::{FirstName, LastName};
#[macro_use]
mod calculate;
struct Age {
    value: i32,
}
fn main() {
    generate_get_value!(FirstName, String);
    generate_get_value!(Age, i32);
    let first = FirstName::new("John".to_string());
    let value_of = first.get_value().clone();
    let age = Age::new(23);
    let age_va = age.get_value().clone();

    dbg!(value_of);
    dbg!(age_va);
    //    trace_macros!(true);
    //    greeting!("Nick", "Heya");
    //    greeting!(test "Heya");
    //    trace_macros!(false);
}
