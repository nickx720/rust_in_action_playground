fn main() {
    dbg!("{}", std::mem::size_of::<Foo>());
    let heap_box: Box<[u8; 4]> = Box::new([1, 2, 3, 4]);
    print(heap_box as Box<dyn Printable>);
}

fn random<const N: usize>(val: &[u8; N]) {
    println!("{}", val.len());
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
impl<const N: usize> Printable for [u8; N] {
    fn stringify(&self) -> String {
        let mut output = String::new();
        for item in self.iter() {
            output.push_str(item.to_string().as_str());
        }
        output
    }
}
fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}
#[repr(C)] // 32 bytes with padding
#[derive(Debug)]
struct Foo {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}
