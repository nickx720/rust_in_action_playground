mod webhook;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::{self, Json},
    App, Error, HttpResponse, HttpServer, Responder,
};
use dotenv;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use url::Url;
use webhook::{read_json_file, WebHookBuilder, WebHookError};

use crate::convert_markdown_file;

use self::webhook::RepoConfig;
// http://danielwelch.github.io/rust-web-service.html
// https://actix.rs/docs/middleware
// https://github.com/actix/examples/blob/master/middleware/request-extensions/src/main.rs
#[derive(Deserialize, Debug)]
struct Config {
    content_type: String,
    insecure_ssl: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct LastResponse {
    code: Option<i32>,
    status: Option<String>,
    message: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Hook {
    active: bool,
    config: Config,
    created_at: String,
    events: Vec<String>,
    id: i32,
    last_response: LastResponse,
    name: String,
    ping_url: String,
    test_url: String,
    #[serde(rename = "type")]
    type_val: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct PushEvent {
    hook: Hook,
    hook_id: i32,
    zen: String,
}

#[derive(Deserialize, Debug)]
struct PullRequest {
    action: String,
    assignee: Assignee,
    enterprise: Enterprise,
    installation: Installation,
    number: i32,
    organization: Organization,
    pull_request: PullRequestObj,
    repository: Repository,
    sender: Sender,
}

#[derive(Deserialize, Debug)]
struct Assignee {
    avatar_url: String,
    deleted: bool,
    email: Option<String>,
    events_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    html_url: String,
    id: i32,
    login: String,
    name: String,
    node_id: String,
    organizations_url: String,
    received_events_url: String,
    repos_url: String,
    site_admin: bool,
    starred_url: String,
    subscriptions_url: String,
    #[serde(rename = "type")]
    type_val: String,
    url: String,
}
#[derive(Deserialize, Debug)]
struct Enterprise {}
#[derive(Deserialize, Debug)]
struct Installation {}
#[derive(Deserialize, Debug)]
struct Organization {}
#[derive(Deserialize, Debug)]
struct PullRequestObj {}

#[derive(Deserialize, Debug)]
struct Repository {}

#[derive(Deserialize, Debug)]
struct Sender {}
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

// TODO On Githook push, generate new api endpoints
// https://docs.github.com/en/rest/repos/webhooks?apiVersion=2022-11-28
// Get list of webhooks,
// Create webhook
// https://andrewlock.net/using-ssh-and-localhost-run-to-test-github-webhooks-locally/
// https://localhost.run/docs/
// TODO create an endpoint for localhost:5000 to point to from proxy
async fn webhook() -> Result<impl Responder, WebHookError> {
    let bearer_token = dotenv::var("GITHUB_TOKEN")?;
    let bearer_token = format!("Bearer {}", bearer_token);
    let webhook_url = read_json_file("./docs/repo.json")?;
    for url in webhook_url {
        // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html
        let headers = generate_headers(&bearer_token);
        let sample = WebHookBuilder::new("web".to_string());
        let webhook_input = sample
            .active(true)
            .events(vec!["push".to_string(), "pull_request".to_string()])
            .url("https://0e56a883fa1d63.lhr.life/engaged".to_string())
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

#[derive(Serialize, Deserialize, Debug)]
struct Contents {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    download_url: String,
}

fn generate_url(url: RepoConfig) -> Result<String, Box<dyn std::error::Error>> {
    let individual_components = Url::parse(&url.base)?;
    let individual_segments = individual_components
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();
    let base_url = format!(
        "https://api.github.com/repos/{}/{}",
        individual_segments[0], individual_segments[1]
    );
    Ok(base_url)
}

fn temp_wrapper(url: String) -> String {
    format!("{}/contents/rust-web-development/markdown-html/docs", url)
}

fn generate_headers(bearer_token: &String) -> header::HeaderMap {
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
    headers
}

//https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28
async fn read_contents_repo() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let bearer_token = dotenv::var("GITHUB_TOKEN")?;
    let bearer_token = format!("Bearer {}", bearer_token);
    let webhook_url = read_json_file("./docs/repo.json")?;
    let mut contents = vec![];
    for url in webhook_url {
        // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html
        let headers = generate_headers(&bearer_token);

        let base_url = generate_url(url)?;
        let url = temp_wrapper(base_url);
        // TODO refactor the following block
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        let content = client.get(url).send().await?.text().await?;
        let content: Vec<Contents> = serde_json::from_str(&content)?;
        //  TODO      Parallelize this, possible optimization
        for cont in &content {
            let desc = client.get(&cont.download_url).send().await?.text().await?;
            match &cont.url {
                url if url.contains(".md") => {
                    let converted = convert_markdown_file(desc).unwrap();
                    contents.push(converted);
                    continue;
                }
                url if url.contains(".yaml") => {
                    let spec = oas3::from_reader(desc.as_bytes()).unwrap();
                    let json_value = oas3::to_json(&spec).unwrap();
                    contents.push(json_value);
                    continue;
                }
                _ => {
                    contents.push(format!("{} file is not supported as of now", &cont.url));
                    continue;
                }
            }
        }
    }
    Ok(HttpResponse::Ok().json(contents))
}

// @TODO figure out why post is not pushing to webhook
async fn from_webhook(push: Json<PushEvent>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    dbg!("Invoked via webhook");
    dbg!(&push);
    Ok(HttpResponse::Ok().body("Hello world"))
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("webhook").to(webhook))
            .service(web::resource("register").to(read_contents_repo))
            .service(
                web::resource("engaged")
                    .wrap(VerifySignature)
                    .post(from_webhook),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
