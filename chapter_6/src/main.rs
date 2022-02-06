mod external;
// mod particle;
mod references;
mod stack;
mod unsafer;
use external::external;
// use particle::particle_main;
use references::references;
use stack::stack_main;
use unsafer::unsafe_test;

static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    //    references();
    // external()
    //stack_main();
    //unsafe_test();
    // particle_main();
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL:     {:p}", &GLOBAL as *const i32);
    println!("local_str:  {:p}", local_str as *const str);
    println!("local_int:  {:p}", &local_int as *const i32);
    println!("boxed_int:  {:p}", Box::into_raw(boxed_int));
    println!("boxed_str:  {:p}", Box::into_raw(boxed_str));
    println!("fn_int:     {:p}", fn_int);
}
