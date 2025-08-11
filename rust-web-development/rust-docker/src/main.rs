use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    run: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Image {
        #[arg(short, long)]
        command: String,
        #[arg(short, long)]
        args: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();
    if let Some(arguments) = args.run {
        dbg!(&arguments);
    }
}
