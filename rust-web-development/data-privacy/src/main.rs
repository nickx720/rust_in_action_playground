use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, middleware, post,
    web::{self, Json},
};
mod db;
use db::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct RequestPayload {
    id: String,
    data: HashMap<String, String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/tokenize")]
async fn tokenize(req_body: Json<RequestPayload>) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&req_body).unwrap())
}
#[post("/detokenize")]
async fn detokenize(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let manager = SqliteConnectionManager::file("dataprivacy.db");
    let pool = Pool::new(manager).unwrap();

    log::info!("starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(tokenize)
            .service(hello)
            .service(detokenize)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
