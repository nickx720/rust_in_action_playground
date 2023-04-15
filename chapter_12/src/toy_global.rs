use rand;

static mut SHUT_DOWN: bool = false;
pub fn toy_global_main() {
    loop {
        unsafe {
            SHUT_DOWN = rand::random();
        }
        print!(".");

        if unsafe { SHUT_DOWN } {
            break;
        };
    }
    println!();
}
