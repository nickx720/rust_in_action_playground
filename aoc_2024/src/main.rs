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
            if part_one.is_some() {
                let output = day_one::partone::part_one(file).unwrap();
                println!("The output of day one first part is {}", output);
            }
            if part_two.is_some() {
                let output = day_one::parttwo::part_two(file).unwrap();
                println!("The output of day one second part is {}", output);
            }
        }
        Some(Commands::Two { part_one, part_two }) => {
            println!("Invoking Day one");
            let file = "./assets/day_two/question.txt";
            if part_one.is_some() {
                let output = day_two::part_one::part_one(file).unwrap();
                println!("The output of day one first part is {}", output);
            }
            if part_two.is_some() {
                let output = day_one::parttwo::part_two(file).unwrap();
                println!("The output of day one second part is {}", output);
            }
        }
        _ => panic!("Invalid day"),
    }
}
