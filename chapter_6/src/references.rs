use std::mem::size_of;

pub static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
pub static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn references() {
    let a = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (as unsigned integer):");
    println!("  location:  {:p}", &a);
    println!("  size:  {:?} bytes", size_of::<usize>());
    println!("  value:  {:?}", &a);
    println!();

    println!("b (a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:  {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to:  {:p}", b);
    println!();

    println!("c (a box for C):");
    println!("  location:  {:p}", &c);
    println!("  size:  {:?} bytes", size_of::<&Box<[u8]>>());
    println!("  points to:  {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!("  location:  {:p}", &B);
    println!("  size:  {:?} bytes", size_of::<[u8; 10]>());
    println!("  value to:  {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("  location:  {:p}", &C);
    println!("  size:  {:?} bytes", size_of::<[u8; 11]>());
    println!("  value to:  {:?}", C);
    println!();
}
