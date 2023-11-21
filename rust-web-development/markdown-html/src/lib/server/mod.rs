use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use std::future::{ready, Ready};

// http://danielwelch.github.io/rust-web-service.html
// https://actix.rs/docs/middleware
// https://github.com/actix/examples/blob/master/middleware/request-extensions/src/main.rs

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
    type Transform = VerifySignatureResp<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifySignatureResp { service }))
    }
}

pub struct VerifySignatureResp<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for VerifySignatureResp<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;
    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        dbg!("{}", req.headers().to_owned());
        self.service.call(req)
    }
}

async fn webhook() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/webhook").wrap(VerifySignature).to(webhook))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
