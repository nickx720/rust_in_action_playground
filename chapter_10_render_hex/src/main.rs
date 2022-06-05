use std::env;

use svg::node::element::path::{Command,Data,Position};
use svg::node::element::{Path,Rectangle};
use svg::Document;

#[derive(Debug,Clone,Copy)]
enum Operation{
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(usize),
}

fn parse(input:&str) -> Vec<Operation>{
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes(){
        let step = match byte{
            b'0'=> Home,
            b'1'..b'9' => {
                let distance = {byte- 0x30} as isize;
                Forward(distance * (HEIGHT/10))
            },
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => Turn Right,
            _ => Noop(byte),

        }
    };
    steps.push(step);
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(default);

    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}
