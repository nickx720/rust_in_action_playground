use num::complex::Complex;

pub fn complex() {
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1,22.2);
    let result = a + b;
    println!("{} + {}i",result.re,result.im)
}

pub fn references() {
    let a = 42;
    let r = &a;
    let b = a + *r;
    print!("a + a is {}",b);
}
