use std::env;

use svg::node::element::path::{Command,Data,Position};
use svg::node::element::{Path,Rectangle};
use svg::Document;

use crate::Operation::{
    Forward,
    Home,
    Noop,
    TurnLeft,
    TurnRight,
};

use crate::Orientation::{
    East,
    North,
    South,
    West
};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

const STROKE_WIDTH: usize = 5;

#[derive(Debug,Clone,Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Debug,Clone,Copy)]
enum Operation{
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist{
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self){
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading{
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x +=distance,
            East => self.x -= distance,
        }
    }

    fn turn_right(&mut self){
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East =>South,
        }
    }
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
