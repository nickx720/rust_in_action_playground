fn main() {
    let x = 42;
    let y = 43;
    let mut var2 = &x;
    var2 = &y;
    dbg!(*var2);
}
