mod webhook;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, App, Error, HttpResponse, HttpServer, Responder,
};
use dotenv;
use reqwest::header;
use serde::Deserialize;
use std::future::{ready, Ready};
use webhook::{read_json_file, WebHookBuilder, WebHookError};
// http://danielwelch.github.io/rust-web-service.html
// https://actix.rs/docs/middleware
// https://github.com/actix/examples/blob/master/middleware/request-extensions/src/main.rs
// Register a repository Done
// Create a webhook Done
// Access the contents of the branch,read up the files and generate markdown and store it

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

// https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28
// Get list of webhooks,
// Create webhook
// https://andrewlock.net/using-ssh-and-localhost-run-to-test-github-webhooks-locally/
// https://localhost.run/docs/
async fn webhook() -> Result<impl Responder, WebHookError> {
    let bearer_token = dotenv::var("GITHUB_TOKEN")?;
    let bearer_token = format!("Bearer {}", bearer_token);
    let webhook_url = read_json_file("./docs/repo.json")?;
    for url in webhook_url {
        // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&bearer_token).unwrap(),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_static("2022-11-28"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0",
            ),
        );
        let sample = WebHookBuilder::new("web".to_string());
        let webhook_input = sample
            .active(true)
            .events(vec!["push".to_string(), "pull_request".to_string()])
            .url("https://example.com/webhook".to_string())
            .content_type("json".to_string())
            .insecure_ssl(0.to_string())
            .builder()
            .to_json();
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        let url = format!("{}/hooks", url.repo);
        let list_of_webhooks = client
            .post(url)
            .json(&webhook_input)
            .send()
            .await?
            .text()
            .await?;
        dbg!(list_of_webhooks);
    }
    Ok(HttpResponse::Ok().body("Hello world!"))
}

async fn read_contents_repo() -> Result<impl Responder, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok())
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/webhook").wrap(VerifySignature).to(webhook))
            .service(web::resource("register").to(read_contents_repo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
