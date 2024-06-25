use beej_examples::{self, showip::show_ip};
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ShowIp { host: String, service: String },
}

fn main() {
    let args = Args::parse();
    let output = match args.command {
        Commands::ShowIp { host, service } => show_ip(host, service),
    };
    println!("The result of compute is {}", output);
}
