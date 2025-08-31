// filepath: /Volumes/SSD/Personal/Rust/learnRust/rust_in_action/rust-web-development/100-exercises-to-learn-rust/exercises/08_futures/08_outro/src/bin/server.rs
use outro_08::run_server; // adjust if your crate name differs

#[tokio::main]
async fn main() {
    if let Err(e) = run_server().await {
        eprintln!("server error: {}", e);
    }
}
