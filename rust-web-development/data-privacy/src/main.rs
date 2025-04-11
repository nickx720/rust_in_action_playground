use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    http::{StatusCode, header::ContentType},
    middleware, post,
    web::{self, Json},
};
use base64::{DecodeError, prelude::*};
use db::{DBError, DataPrivacyStore, Pool, get_token, initialize_db, insert_token};
use encryption::{EncryptionError, decrypt_data, encrypt_data};
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::ParseIntError};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum DeTokenError {
    #[error("ParseInt Failure")]
    ParseInt(#[from] ParseIntError),
    #[error("DB Error")]
    DB(#[from] DBError),
    #[error("Base 64 Decode")]
    Decode(#[from] DecodeError),
    #[error("Encryption Error")]
    Encrypt(#[from] EncryptionError),
    #[error("JSON parsing error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Random Error")]
    Other,
}
impl actix_web::error::ResponseError for DeTokenError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/tokenize")]
async fn tokenize(
    req_body: Json<TokenPayload>,
    pool: web::Data<Pool>,
    key: web::Data<[u8; 32]>,
) -> Result<impl Responder, DeTokenError> {
    let token: Result<HashMap<String, String>, DeTokenError> = req_body
        .data
        .iter()
        .map(|(index, tokenize)| {
            let token = encrypt_data(tokenize, &key)?;
            let string = BASE64_STANDARD.encode(token);
            Ok((index.clone(), string))
        })
        .collect();
    let token = token?;
    let token = serde_json::to_string(&token)?;
    let token = DataPrivacyStore::new(req_body.id.parse::<u32>().unwrap(), token);
    match insert_token(&pool, token).await {
        Ok(_val) => {
            let tokenized_value = get_token(&pool, req_body.id.parse::<u32>()?).await.unwrap();
            let body = serde_json::to_string(&tokenized_value)?;
            Ok(HttpResponse::Ok().body(body))
        }
        Err(e) => Err(DeTokenError::Other),
    }
}
#[post("/detokenize")]
async fn detokenize(
    req_body: Json<DeTokenPayload>,
    pool: web::Data<Pool>,
    key: web::Data<[u8; 32]>,
) -> Result<impl Responder, DeTokenError> {
    let id = req_body.id.parse::<u32>()?;
    let retrieved_token = get_token(&pool, id).await?;
    if let Some(token) = retrieved_token.get_data() {
        let original_token: Result<HashMap<String, String>, DeTokenError> = token
            .iter()
            .map(|item| {
                let (index, val) = item;
                if let Some(temp_token) = val.as_str() {
                    let string = BASE64_STANDARD.decode(temp_token)?;
                    let detoken = decrypt_data(string.as_ref(), &key)?;
                    Ok((index.clone(), detoken))
                } else {
                    Err(DeTokenError::Other)
                }
            })
            .collect();
        let original_token = original_token?;
        let body = serde_json::to_string(&original_token)?;
        Ok(HttpResponse::Ok().body(body))
    } else {
        Ok(HttpResponse::BadRequest().body("Bad Request"))
    }
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
            .service(detokenize)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
