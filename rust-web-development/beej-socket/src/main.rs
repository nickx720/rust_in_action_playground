use beej_examples::{self, showip::show_ip};
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    show_ip: bool,
}

fn main() {
    match Args::parse() {
        Args { show_ip } => show_ip(),
        _ => panic!("Unsupported"),
    }
}
