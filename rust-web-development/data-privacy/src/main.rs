use actix_web::{
    App, HttpResponse, HttpServer, Responder, ResponseError, get,
    http::StatusCode,
    middleware, post,
    web::{self, Json},
};
mod db;
use db::{DataPrivacyStore, Pool, initialize_db, insert_token};
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
async fn tokenize(req_body: Json<RequestPayload>, pool: web::Data<Pool>) -> impl Responder {
    let token = DataPrivacyStore::new(
        req_body.id.parse::<u32>().unwrap(),
        "yes".to_string(),
        "no".to_string(),
    );
    match insert_token(&pool, token).await {
        Ok(val) => HttpResponse::Ok().body(val.to_string()),
        Err(e) => {
            dbg!(e);
            HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .finish()
        }
    }
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
    let _ = initialize_db(&pool).await;

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
