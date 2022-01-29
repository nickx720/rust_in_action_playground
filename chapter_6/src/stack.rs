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
}
