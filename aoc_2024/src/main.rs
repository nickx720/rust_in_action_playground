use clap::{Parser, Subcommand};
mod day_one;
mod day_two;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    One {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Two {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::One { part_one, part_two }) => {
            println!("Invoking Day one");
            let file = "./assets/day_one/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_two::part_one::part_one(file).unwrap();
                        println!("The output of day two first part is {}", output);
                    }
                }
                _ => println!("Ignoring day one part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_one::parttwo::part_two(file).unwrap();
                        println!("The output of day one second part is {}", output);
                    }
                }
                _ => println!("Ignoring day one part two"),
            }
        }
        Some(Commands::Two { part_one, part_two }) => {
            println!("Invoking Day Two");
            let file = "./assets/day_two/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_two::part_one::part_one(file).unwrap();
                        println!("The output of day two first part is {}", output);
                    }
                }
                _ => println!("Ignoring day one part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_two::part_two::part_two(file).unwrap();
                        println!("The output of day two second part is {}", output);
                    }
                }
                _ => println!("Ignoring day two part two"),
            }
        }
        _ => panic!("Invalid day"),
    }
}
