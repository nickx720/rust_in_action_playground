mod external;
mod references;
use external::external;
use references::references;

fn main() {
    //    references();
    // external()
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
    // println!("a : {} ({:p})", a, a_ptr);

    // Unsafe pointer from integer
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
