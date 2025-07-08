use std::{io::Read, process::Command};

//Step 3
//
//In this step your goal is to support the -n option, which does this (from the man page):
//
// -n number, --max-args=number
//             Set the maximum number of arguments taken from standard
//             input for each invocation of utility.  An invocation of
//             utility will use less than number standard input arguments
//             if the number of bytes accumulated (see the -s option)
//             exceeds the specified size or there are fewer than number
//             arguments remaining for the last invocation of utility.  The
//             current default value for number is 5000.
//
//You can then test this like so:

//% ls | ccxargs -n 1 cat
//This is file 1
//This is file 2
//This is file 3
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    command: Option<Vec<String>>,

    #[arg(short, long)]
    n: Option<u16>,

    #[arg(short, long)]
    max_args: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let stdin_input = buffer.split_whitespace().collect::<Vec<&str>>();

    // You can check the value provided by positional arguments, or option arguments

    if let Some(cmd) = cli.command {
        let command = cmd.join(" ");
        if let Some(number) = cli.n {
            let number = number as usize;
            for (index, item) in stdin_input.iter().skip(number).enumerate() {
                // TODO get the elements for n
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg(format!("{} {}", command, *item))
                    .spawn()
                    .expect("failed to execute")
                    .stdout;
            }
        } else {
            let item = stdin_input.join("");
            let _ = Command::new("sh")
                .arg("-c")
                .arg(format!("{} {}", command, item))
                .spawn()
                .expect("failed to execute")
                .stdout;
        }
    }
}
