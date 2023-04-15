
//use core::arch::asm;
//fn main() {
//    unsafe {
//        asm!("int 42");
//    }
//}
//use std::process;
//use std::thread::sleep;
//use std::time;
mod shutdown;
mod toy_global;

fn main() {
    //    let delay = time::Duration::from_secs(1);
    //    let pid = process::id();
    //    println!("{pid}");
    //
    //    for i in 1..=60 {
    //        sleep(delay);
    //        println!("{i}");
    //    }
    toy_global::toy_global_main();
}
