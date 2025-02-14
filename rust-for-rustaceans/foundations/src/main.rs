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
    let mut s = Box::new(42);
    replace_with_84(&mut s);
    let mut x = Box::new(42);
    let r = &x;
    if rand() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r);
    }
    let mut x = Box::new(42);
    let mut z = &x;
    for i in 0..100 {
        println!("{}", z);
        x = Box::new(i);
        z = &x;
    }
    println!("{}", z);
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s);
}
fn rand() -> f32 {
    2.5
}

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

fn replace_with_84(s: &mut Box<i32>) {
    let was = std::mem::take(s);
    *s = was;
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_eq!(*r, 42);
    assert_ne!(*r, 84);
}

struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}
impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}
