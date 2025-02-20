fn main() {
    dbg!("{}", std::mem::size_of::<Foo>());
    let new_val: [u8; 4] = [1, 2, 3, 4];
    let output = &new_val[..].stringify();
    println!("I am here {}", output);
}

trait Printable {
    fn stringify(&self) -> String;
}
impl Printable for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}
impl Printable for u8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}
impl Printable for [u8] {
    fn stringify(&self) -> String {
        self.len().to_string()
    }
}
impl Printable for &[u8] {
    fn stringify(&self) -> String {
        self.to_owned().stringify()
    }
}
// TODO get it to work with [u8:4]
fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}
// #[repr(align(1024))] 1024 bytes
// #[repr(packed)] 16 bytes
#[repr(C)] // 32 bytes with padding
#[derive(Debug)]
struct Foo {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}
