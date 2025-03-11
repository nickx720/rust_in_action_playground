use actix_web::{
    App, HttpResponse, HttpServer, Responder, ResponseError, get,
    http::StatusCode,
    middleware, post,
    web::{self, Json},
};
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use rand::Rng;
mod db;
use db::{DataPrivacyStore, Pool, initialize_db, insert_token};
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

#[derive(Serialize, Deserialize, Debug)]
struct RequestPayload {
    id: String,
    data: HashMap<String, String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn encrypt_data(data: &str, key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes()).unwrap();
    (nonce.to_vec(), ciphertext)
}

fn decrypt_data(encrypted_data: &[u8], key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let (nonce, ciphertext) = encrypted_data.split_at(12); // Extract nonce
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .unwrap();
    String::from_utf8(plaintext).unwrap()
}

#[post("/tokenize")]
async fn tokenize(
    req_body: Json<RequestPayload>,
    pool: web::Data<Pool>,
    key: web::Data<[u8; 32]>,
) -> impl Responder {
    let original = serde_json::to_string(&req_body.data).unwrap();
    let mut nonce = Vec::new();
    for (_index, tokenize) in req_body.data.to_owned().iter_mut() {
        let token = encrypt_data(&tokenize, &key);
        nonce = token.0;
        *tokenize = String::from_utf8(token.1).expect("Conversion failed");
    }
    let token = serde_json::to_string(&req_body.data).unwrap();
    let token = DataPrivacyStore::new(req_body.id.parse::<u32>().unwrap(), original, token, nonce);
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
