use crossbeam::channel::unbounded;
use std::thread;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

pub fn channels_intro_main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    select! {
        recv(rx) -> msg => println!("{:?}",msg),
    }
}
