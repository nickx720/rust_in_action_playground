use std::{io::Read, process::Command};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    command: Option<Vec<String>>,
}

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let stdin_input = buffer.split_whitespace().collect::<Vec<&str>>();
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments

    if let Some(cmd) = cli.command {
        let command = cmd.join(" ");
        for item in &stdin_input {
            let _ = Command::new("sh")
                .arg("-c")
                .arg(format!("{} {}", command, *item))
                .spawn()
                .expect("failed to execute")
                .stdout;
        }
    }
}
