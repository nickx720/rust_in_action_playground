use std::error::Error;

use reqwest;

pub fn reqwestmain() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{content}");

    Ok(())
}
