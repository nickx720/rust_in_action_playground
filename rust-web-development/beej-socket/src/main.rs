use std::net::IpAddr;

use beej_examples::{
    self, listener::socketlistener, pollexample::pollexample, pollserver::pollserver,
    selectexample, showip::show_ip, streamclient::streamclient, streamserver::streamserver,
    talker::sockettalker,
};
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
    StreamClient {
        host: String,
    },
    Listener {
        port: u16,
        #[arg(short, long, value_enum, default_value_t = Family::Unspecified)]
        family: Family,
    },
    Talker {
        host: IpAddr,
        port: u16,
        message: String,
    },
    PollExample,
    PollServer {
        port: u16,
    },
    SelectExample {
        port: u16,
    },
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
        Commands::StreamClient { host } => streamclient(host),
        Commands::Listener { port, family } => socketlistener(port, family),
        Commands::Talker {
            host,
            port,
            message,
        } => sockettalker(host, port, message),
        Commands::PollExample => pollexample(),
        Commands::PollServer { port } => pollserver(port),
        Commands::SelectExample { port } => selectexample::selectexample(port),
    };
}
