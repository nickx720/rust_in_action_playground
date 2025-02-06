fn main() {
    let x = 42;
    let y = 43;
    let mut var2 = &x;
    var2 = &y;
    dbg!(*var2);
    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
        // z goes out scope
    }
    let x2 = x1;
    //    let y2 = y1; won't work since y1 no longer owns the pointer
}
