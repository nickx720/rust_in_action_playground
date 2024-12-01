use clap::{Parser, Subcommand};
mod day_one;
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
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::One { part_one, part_two }) => {
            println!("Invoking Day one");
            if part_one.is_some() {
                day_one::partone::part_one();
            }
            if part_two.is_some() {
                day_one::parttwo::part_two()
            }
        }
        _ => panic!("Invalid day"),
    }
}
