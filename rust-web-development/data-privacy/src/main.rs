use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::StatusCode,
    middleware, post,
    web::{self, Json},
};
use base64::prelude::*;
use db::{DataPrivacyStore, Pool, get_token, initialize_db, insert_token};
use encryption::{decrypt_data, encrypt_data};
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod db;
mod encryption;

#[derive(Serialize, Deserialize, Debug)]
struct TokenPayload {
    id: String,
    data: HashMap<String, String>,
}
#[derive(Deserialize)]
struct DeTokenPayload {
    id: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/tokenize")]
async fn tokenize(
    req_body: Json<TokenPayload>,
    pool: web::Data<Pool>,
    key: web::Data<[u8; 32]>,
) -> impl Responder {
    let token = req_body
        .data
        .iter()
        .map(|(index, tokenize)| {
            let token = encrypt_data(tokenize, &key).unwrap();
            let string = BASE64_STANDARD.encode(token);
            (index.clone(), string)
        })
        .collect::<HashMap<String, String>>();
    let token = serde_json::to_string(&token).unwrap();
    let token = DataPrivacyStore::new(req_body.id.parse::<u32>().unwrap(), token);
    match insert_token(&pool, token).await {
        Ok(_val) => {
            let tokenized_value = get_token(&pool, req_body.id.parse::<u32>().unwrap())
                .await
                .unwrap();
            let body = serde_json::to_string(&tokenized_value).unwrap();
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            dbg!(e);
            HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .finish()
        }
    }
}
#[post("/detokenize")]
async fn detokenize(
    req_body: Json<DeTokenPayload>,
    pool: web::Data<Pool>,
    key: web::Data<[u8; 32]>,
) -> impl Responder {
    let id = req_body.id.parse::<u32>().unwrap();
    let retrieved_token = get_token(&pool, id).await.unwrap();
    let original_token = retrieved_token
        .get_data()
        .unwrap()
        .iter()
        .map(|item| {
            // TODO test
            let (index, val) = item;
            let temp_token = val.as_str().unwrap();
            dbg!(&temp_token);
            let string = BASE64_STANDARD.decode(temp_token).unwrap();
            let detoken = decrypt_data(string.as_ref(), &key);
            (index.clone(), detoken)
        })
        .collect::<HashMap<String, String>>();
    let body = serde_json::to_string(&original_token).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let manager = SqliteConnectionManager::file("dataprivacy.db");
    let pool = Pool::new(manager).unwrap();
    let key: [u8; 32] = rand::rng().random::<[u8; 32]>();
    let _ = initialize_db(&pool).await;

    log::info!("starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(key))
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
