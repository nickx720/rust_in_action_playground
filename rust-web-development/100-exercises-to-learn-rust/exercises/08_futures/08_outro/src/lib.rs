// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use ticket_fields::{TicketDescription, TicketTitle};
use url::form_urlencoded;

use crate::ticket::{Status, Ticket, TicketDraft, TicketId, TicketStore};
mod ticket;

#[derive(Deserialize, Debug)]
struct CommonFields {
    title: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct ParseBody {
    #[serde(flatten)]
    common: CommonFields,
}

#[derive(Deserialize, Debug)]
struct TicketPatch {
    #[serde(flatten)]
    common: Option<CommonFields>,
    extra_field: Option<Status>,
}

#[derive(Serialize, Debug)]
struct TicketCreated {
    id: u64,
}

async fn create(
    req: Request<Body>,
    ticket: Arc<Mutex<TicketStore>>,
) -> Result<Response<Body>, Infallible> {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body: ParseBody =
        serde_json::from_slice::<ParseBody>(&body).expect("Parsing failed, validate");
    let title = TicketTitle::try_from(body.common.title);
    let description = TicketDescription::try_from(body.common.description);
    match (title, description) {
        (Ok(title), Ok(description)) => {
            let ticket_new = TicketDraft { title, description };
            let mut ticket = ticket.lock().unwrap();
            let ticket_id = ticket.add_ticket(ticket_new);
            let output = TicketCreated {
                id: ticket_id.get(),
            };
            let created = serde_json::to_string(&output).unwrap();
            let mut created = Response::new(Body::from(created));
            *created.status_mut() = StatusCode::CREATED;
            Ok(created)
        }
        _ => {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
async fn read(
    req: Request<Body>,
    ticket: Arc<Mutex<TicketStore>>,
) -> Result<Response<Body>, Infallible> {
    if let Some(q) = req.uri().query() {
        let params: HashMap<String, String> =
            form_urlencoded::parse(q.as_bytes()).into_owned().collect();
        if params.is_empty() {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(not_found);
        }
        let ticket = ticket.lock().unwrap();
        let question_id = params.get("question").unwrap();
        let ticket_id = TicketId::set(question_id.parse::<u64>().unwrap());
        let ticket = ticket.get(ticket_id).unwrap();
        let created = serde_json::to_string(ticket).unwrap();
        let mut read = Response::new(Body::from(created));
        *read.status_mut() = StatusCode::OK;
        return Ok(read);
    } else {
        let mut bad_request = Response::new(Body::from("Not Found"));
        *bad_request.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(bad_request);
    }
}
async fn patch(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body: TicketPatch =
        serde_json::from_slice::<TicketPatch>(&body).expect("Parsing failed, validate");
    Ok(Response::new(Body::from("Patch stub")))
}
async fn router(
    req: Request<Body>,
    ticket: Arc<Mutex<TicketStore>>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/create") => create(req, ticket.clone()).await,
        (&Method::GET, "/read") => read(req, ticket.clone()).await,
        (&Method::PATCH, "/update") => patch(req).await,
        _ => {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // TODO pass ticket_store to routes
    let mut ticket = Arc::new(Mutex::new(TicketStore::new()));
    let make_svc = make_service_fn(|_conn| {
        let ticket = ticket.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| router(req, ticket.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
}
