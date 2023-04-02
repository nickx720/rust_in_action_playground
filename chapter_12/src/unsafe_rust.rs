use core::arch::asm;
pub fn example_one() {
    unsafe {
        asm!("int 42");
    }
}
