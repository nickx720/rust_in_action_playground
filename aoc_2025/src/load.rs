use dotenv;
use std::{env, fs};

use reqwest::blocking;

pub fn load(day: &str) -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let session = env::var("AOC_SESSION")?;
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let client = blocking::Client::builder()
        .user_agent("github.com/nickx720")
        .build()?;
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .error_for_status()?;
    let text = response.text()?;
    fs::create_dir(format!("./assets/day{}", day))?;
    fs::write(format!("./assets/day{}/{}.txt", day, day), text)?;
    Ok(())
}
