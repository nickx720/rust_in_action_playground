use std::mem::drop;
// Implicit conversion
fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}

// Explicit Conversion
fn is_strong_explicit<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

pub fn stack_main() {
    let pw = "justok";
    let is_strong = is_strong_explicit(pw);
    dbg!("The output is {:?}", is_strong);
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    // assinging to stack
    let result_1 = *a + *b + *c;
    drop(a);
    let d = Box::new(2);
    let result_2 = *b + *c + *d;
    println!("The output is {},{}", result_1, result_2);
}
