mod asyncexample;
mod question;
use warp::Filter;
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    //question::question_main();
//    //    let response = asyncexample::async_example();
//    //    response.await.expect("Future didn't complete");
//    Ok(())
//}
//
#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .and(warp::path::param())
        .map(|name: String| format!("Hello {name}!"));

    warp::serve(hello).run(([127, 0, 0, 1], 1337)).await;
}
