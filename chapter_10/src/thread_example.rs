use std::{thread, time};

pub fn thread_example_main() {
    let start = time::Instant::now();

    let handler = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}

pub fn thread_two_threads_example() {
    let start = time::Instant::now();

    let handler_1 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    let handler_2 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler_1.join().unwrap();
    handler_2.join().unwrap();

    let finish = time::Instant::now();

    println!("{:?}", finish.duration_since(start));
}
