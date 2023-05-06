use std::collections::HashMap;

pub async fn async_example() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    dbg!("Hello");
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("I am here");
    println!("{:#?}", resp);
    Ok(resp)
}
