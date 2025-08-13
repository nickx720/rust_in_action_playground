use std::{
    io::{self, Write},
    process::Command,
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    run: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(short, long)]
        command: String,
        #[arg(short, long,num_args=1..)]
        args: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();
    if let Some(arguments) = args.run {
        match arguments {
            Commands::Run { command, args } => {
                let output = Command::new(command).arg(args.join(" ")).output().unwrap();
                let _ = io::stdout().write_all(&output.stdout);
                let _ = io::stderr().write_all(&output.stderr);
            }
        }
    }
}
