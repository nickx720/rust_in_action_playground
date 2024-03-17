pub fn base_greeting_fn(name: &str, greeting: &str) {
    println!("{} , {}", greeting, name);
}

macro_rules! greeting {
    ($name:literal) => {
        base_greeting_fn($name, "Hello")
    };
    ($name:literal,$greeting:literal) => {
        base_greeting_fn($name, $greeting)
    };
    (test $name:literal) => {{
        log_syntax!("the name passed is ", $name);
        base_greeting_fn($name, "Hello")
    }};
}
