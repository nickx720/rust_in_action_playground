use dotenv;
use std::{env, fs};

use reqwest::blocking;

pub fn load(day: &str) -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let dir = format!("./assets/day{}", day);
    let file = format!("{}/{}.txt", dir, day);

    // Skip network work when the input is already cached on disk.
    if fs::metadata(&file).is_ok() {
        return Ok(());
    }

    let session = env::var("AOC_SESSION")?;
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let client = blocking::Client::builder()
        .user_agent("github.com/nickx720")
        .build()?;
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .error_for_status()?;
    let text = response.text()?;

    fs::create_dir_all(&dir)?;
    fs::write(&file, text)?;
    Ok(())
}
