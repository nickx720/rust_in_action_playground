fn main() {
    let x = 42;
    let y = 43;
    let mut var2 = x;
    // var2 = &y;
    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
        // z goes out scope
    }
    let x2 = x1;
    //    let y2 = y1; won't work since y1 no longer owns the pointer
    cache(&84, &mut var2);
}

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}
