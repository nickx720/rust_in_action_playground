use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    show_ip: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}", args.show_ip);
}
