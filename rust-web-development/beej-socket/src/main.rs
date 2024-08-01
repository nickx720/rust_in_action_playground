use beej_examples::{self, showip::show_ip, streamserver::streamserver};
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
        #[arg(short, long, value_enum, default_value_t = Family::Unspecified)]
        family: Family,
        #[arg(short, long, default_value = "http")]
        service: String,
    },
    StreamServer,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::ShowIp {
            host,
            family,
            service,
        } => show_ip(host, family, service).expect("Something went wrong displaying ip"),
        Commands::StreamServer => streamserver(),
    };
}
