use beej_examples::{self, showip::show_ip};
use clap::{Parser, Subcommand};
use types::Family;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ShowIp {
        host: String,
        family: Family,
        service: String,
    },
}

fn main() {
    let args = Args::parse();
    let output = match args.command {
        Commands::ShowIp {
            host,
            family,
            service,
        } => show_ip(host, family, service).expect("Something went wrong displaying ip"),
    };
}
