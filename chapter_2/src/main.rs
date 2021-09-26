mod abs;
mod complex;
mod mandelbrot;
mod quote;
mod needle;

use std::fs::File;
use std::io;
use std::io::BufReader; // buffered I/O which reduces system calls to IO
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};
use mandelbrot::{calculate_mandelbrot,render_mandelbrot};

fn process_lines<T:BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines(){
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}",line),
            None => (),
        }
    }
}

fn main() {
//    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);
//    render_mandelbrot(mandelbrot);
let args = App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
    .arg(Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .required(false))
    .get_matches();

let pattern = args.value_of("pattern").unwrap();
let re = Regex::new(pattern).unwrap();

let input = args.value_of("input").unwrap_or("-");

if input == "-" {
    let stdin = io::stdin();
    let reader = stdin.lock();
    process_lines(reader, re);
} else {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    process_lines(reader, re);
}

}
