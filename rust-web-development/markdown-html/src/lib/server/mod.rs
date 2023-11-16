use std::future::{ready, Ready};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    get, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

// http://danielwelch.github.io/rust-web-service.html
// https://actix.rs/docs/middleware

#[derive(Deserialize)]
struct PushEvent {
    #[serde(rename = "ref")]
    reference: String,
}

struct VerifySignature;

impl<S, B> Transform<S, ServiceRequest> for VerifySignature
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifySignature<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifySignature { service }))
    }
}

#[get("/webhook")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
