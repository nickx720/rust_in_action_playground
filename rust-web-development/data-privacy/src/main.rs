use std::collections::HashMap;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::Json};
use serde::{Deserialize, Serialize};

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
    HttpServer::new(|| {
        App::new()
            .service(tokenize)
            .service(hello)
            .service(detokenize)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
