use std::{io::Read, process::Command};

//Step 4
//
//In this step your goal is to support the -P option, which does this (from the man page):
//
//     -P maxprocs, --max-procs=maxprocs
//             Parallel mode: run at most maxprocs invocations of utility
//             at once.  If maxprocs is set to 0, xargs will run as many
//             processes as possible.
//
//To test this I suggest creating a text file with a list of URLs in it, say urls.txt, then use ccxargs to invoke curl to download the pages.
//
//% cat urls.txt | ccxargs -n 1 -P 1 curl
//
//Which will dump the content of the websites you hit to your console. You might like to time it.
//
//Then run the test again with a higher value of P to see the overall time reduced as the curl requests are sent concurrently.
//
//% cat urls.txt | ccxargs -n 1 -P 10 curl
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    command: Option<Vec<String>>,

    #[arg(short = 'n', long)]
    max_args: Option<u16>,
    #[arg(short = 'P', long)]
    max_procs: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let stdin_input = buffer.split_whitespace().collect::<Vec<&str>>();

    // You can check the value provided by positional arguments, or option arguments

    if let Some(cmd) = cli.command {
        let command = cmd.join(" ");
        if let Some(number) = cli.max_args {
            let number = number as usize;
            for item in stdin_input.chunks(number) {
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg(format!("{} {}", command, item.join(" ")))
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
