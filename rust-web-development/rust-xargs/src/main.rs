// In this step your goal is to build the command ccxargs that will take a whitespace separated set of strings from standard in and convert them into command line arguments that can be passed to a command (referred to as utility in the man page quoted above).

//You can test your code using, this command below to create three text files we can use for testing:
//
//% for i in {1..3}; do echo "This is file ${i}" > test-${i}.txt; done;
//
//Then in the same directory we can use ls to create a whitespace separated list of files and pipe that into our ccxargs program which we will tell to run the command cat with each of the items in the list as the argument to cat:
//
//% ls | ccxargs cat
//This is file 1
//This is file 2
//This is file 3
//
//This is the equivalent of having done:
//
//% cat test-1.txt test-2.txt test-3.txt
//This is file 1
//This is file 2
//This is file 3

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
    if let Some(cmd) = cli.command.as_deref() {
        for command in cmd {
            for item in &stdin_input {
                // dbg!("I am ", command, item);

                Command::new("sh")
                    .arg("-c")
                    .arg(format!("{} {}", command, *item))
                    .output()
                    .expect("failed to execute")
                    .stdout;
            }
        }
    }
}
