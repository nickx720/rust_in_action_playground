use std::env;
use std::thread;

enum Work {
    Task((usize, u8)),
    Finished,
}

fn parse_byte(byte: u8) -> Operation {
    match byte {
        b'0' => Home,
        b'1'..=b'9' => {
            let distance = (byte - 0x30) as isize;
            Forward(distance * (HEIGHT / 10))
        }
        b'a' | b'b' | b'c' => TurnLeft,
        b'd' | b'e' | b'f' => TurnRight,
        _ => Noop(byte),
    }
}

use crossbeam::channel::unbounded;
pub fn threadpool_main() {
    todo!()
}
