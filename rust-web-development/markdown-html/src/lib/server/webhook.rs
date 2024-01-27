use std::fs;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(thiserror::Error, Debug)]
pub enum WebHookError {
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
pub enum ReadingJSONError {
    #[error("File Read Error")]
    FileReadError(#[from] std::io::Error),

    #[error("Serde JSON Parse error")]
    ParsingError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoConfig {
    pub repo: String,
    pub base: String,
}

type ArrayRepoConfig = Vec<RepoConfig>;

pub fn read_json_file(path: &str) -> Result<ArrayRepoConfig, ReadingJSONError> {
    let content = fs::read_to_string(path)?;
    let repo_config: ArrayRepoConfig = serde_json::from_str(&content)?;
    Ok(repo_config)
}

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
pub struct Webhook {
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
    pub fn to_json(self) -> Value {
        json!(self)
    }
}

#[derive(Default)]
pub struct WebHookBuilder {
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
