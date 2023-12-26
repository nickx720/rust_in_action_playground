use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    get,
    http::StatusCode,
    post, web, App, Error, HttpResponse, HttpServer, Responder, ResponseError,
};
use dotenv;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    future::{ready, Ready},
};

// http://danielwelch.github.io/rust-web-service.html
// https://actix.rs/docs/middleware
// https://github.com/actix/examples/blob/master/middleware/request-extensions/src/main.rs
// Register a repository
// Create a webhook
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

#[derive(thiserror::Error, Debug)]
enum WebHookError {
    #[error("Generic Error")]
    InternalParseError(#[from] anyhow::Error),
    #[error("Couldn't parse GITHUB_TOKEN")]
    DotEnvError(#[from] dotenv::Error),
    #[error("Couln't Fetch Data")]
    GetError(#[from] reqwest::Error),
    #[error("JSON Error")]
    JSONError(#[from] ReadingJSONError),
}

impl ResponseError for WebHookError {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::DotEnvError(_) => StatusCode::NOT_FOUND,
            Self::InternalParseError(_) => StatusCode::NOT_FOUND,
            Self::GetError(_) => StatusCode::FORBIDDEN,
            Self::JSONError(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
enum ReadingJSONError {
    #[error("File Read Error")]
    FileReadError(#[from] std::io::Error),

    #[error("Serde JSON Parse error")]
    ParsingError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
struct RepoConfig {
    repo: String,
}

type ArrayRepoConfig = Vec<RepoConfig>;

fn read_json_file(path: &str) -> Result<ArrayRepoConfig, ReadingJSONError> {
    let content = fs::read_to_string(path)?;
    let repo_config: ArrayRepoConfig = serde_json::from_str(&content)?;
    Ok(repo_config)
}

// https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

#[derive(Serialize, Deserialize)]
struct Config {
    url: String,
    content_type: String,
    insecure_ssl: String,
}
impl Config {
    fn new(url: String, content_type: String, insecure_ssl: String) -> Self {
        Config {
            url,
            content_type,
            insecure_ssl,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Webhook {
    name: String,
    active: bool,
    events: Vec<String>,
    config: Config,
}
impl Webhook {
    fn new(name: String, active: bool, events: Vec<String>, config: Config) -> Self {
        Webhook {
            name,
            active,
            events,
            config,
        }
    }
}
// impl builder pattern for webhook config
impl Webhook {
    fn builder() -> WebHookBuilder {
        WebHookBuilder::default()
    }
    pub fn toJson(self) -> Value {
        json!(self)
    }
}

#[derive(Default)]
struct WebHookBuilder {
    name: String,
    active: bool,
    events: Vec<String>,
    url: String,
    content_type: String,
    insecure_ssl: String,
}
impl WebHookBuilder {
    pub fn new(name: String) -> WebHookBuilder {
        WebHookBuilder {
            name: name,
            ..Default::default()
        }
    }
    pub fn active(mut self, active: bool) -> WebHookBuilder {
        self.active = active;
        self
    }
    pub fn events(mut self, events: Vec<String>) -> WebHookBuilder {
        self.events = events;
        self
    }
    pub fn url(mut self, url: String) -> WebHookBuilder {
        self.url = url;
        self
    }
    pub fn content_type(mut self, content_type: String) -> WebHookBuilder {
        self.content_type = content_type;
        self
    }
    pub fn insecure_ssl(mut self, insecure_ssl: String) -> WebHookBuilder {
        self.insecure_ssl = insecure_ssl;
        self
    }
    pub fn builder(self) -> Webhook {
        let config = Config::new(self.url, self.content_type, self.insecure_ssl);
        let webhook = Webhook::new(self.name, self.active, self.events, config);
        webhook
    }
}

// @TODO Create a webhook using reqwest
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
            .toJson();
        let sample = json!({
           "name":"web",
           "active":true,
           "events":[
              "push",
              "pull_request"
           ],
           "config":{
              "url":"https://example.com/webhook",
              "content_type":"json",
              "insecure_ssl":"0"
           }
        });
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        let url = format!("{}/hooks", url.repo);
        let list_of_webhooks = client.post(url).json(&sample).send().await?.text().await?;
        dbg!(list_of_webhooks);
    }
    Ok(HttpResponse::Ok().body("Hello world!"))
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
