use crossbeam::channel::unbounded;
use std::thread;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

pub fn sending_msg_spawned_channel() {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => responses_tx.send(ConnectivityCheck::Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(ConnectivityCheck::Ping).unwrap();
    }
    requests_tx.send(ConnectivityCheck::Pang).unwrap();

    for _ in 0..n_messages {
        select! {
            recv(responses_rx) -> msg => println!("{:?}",msg),
        }
    }
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
metadata
