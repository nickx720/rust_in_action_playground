#![warn(clippy::all)]

use clap::Parser;
use handle_errors::return_error;
use warp::{http::Method, Filter};

use tracing_subscriber::fmt::format::FmtSpan;
mod profanity;
mod routes;
mod store;
mod types;

#[derive(Debug, Default, serde::Deserialize, PartialEq, Parser)]
#[clap(author,version,about,long_about = None)]
struct Args {
    #[clap(short, long, default_value = "warn")]
    log_level: String,
    #[clap(long, default_value = "localhost")]
    database_host: String,
    #[clap(long, default_value = "5432")]
    database_port: u16,
    #[clap(long, default_value = "some-postgres")]
    database_name: String,
    #[clap(long, default_value = "8080")]
    port: u16,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},practical_rust_book={},warp={}",
            args.log_level, args.log_level, args.log_level
        )
    });
    //    let store =     store::Store::new("postgres://postgres:mysecretpassword@localhost:5432/postgres").await;
    let store = store::Store::new(&format!(
        "postgres://{}:{}/{}",
        args.database_host, args.database_port, args.database_name
    ))
    .await;
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions).with(warp::trace(|info| {
            tracing::info_span!("get questions request",method = %info.method(),path = %info.path(),id = %uuid::Uuid::new_v4())
        }));

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let add_answer = warp::post()
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(add_answer)
        .or(delete_question)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], args.port)).await;
}
