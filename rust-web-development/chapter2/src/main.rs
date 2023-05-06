mod asyncexample;
mod question;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //question::question_main();
    dbg!("hello 1");
    let response = asyncexample::async_example();
    response.await.expect("Future didn't complete");
    dbg!("hello 3");
    Ok(())
}
