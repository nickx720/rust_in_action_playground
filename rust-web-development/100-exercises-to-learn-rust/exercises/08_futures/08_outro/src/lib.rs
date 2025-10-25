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
//
// Service trait + pure functions example:

//pub trait TicketRepo {
//    fn add(&mut self, draft: TicketDraft) -> TicketId;
//    fn get(&self, id: TicketId) -> Option<Ticket>;
//    fn update(&mut self, id: TicketId, patch: TicketPatchData) -> Option<Ticket>;
//}
//
//pub fn create_ticket(repo: &mut impl TicketRepo, draft: TicketDraft) -> TicketId {
//    repo.add(draft)
//}
//pub fn read_ticket(repo: &impl TicketRepo, id: TicketId) -> Option<Ticket> {
//    repo.get(id)
//}
//pub fn patch_ticket(repo: &mut impl TicketRepo, id: TicketId, patch: TicketPatchData) -> Option<Ticket> {
//    repo.update(id, patch)
//}
//
//HTTP handler wiring example:
//
//async fn create_handler(req: Request<Body>, repo: Arc<Mutex<dyn TicketRepo + Send>>) ->
//Result<Response<Body>, Infallible> {
//    let body = serde_json::from_slice::<ParseBody>(&hyper::body::to_bytes(req.into_body()).await.unwrap()).
//unwrap();
//    let id = create_ticket(&mut *repo.lock().await, TicketDraft { title: body.common.title.into(),
//description: body.common.description.into() });
//    let mut resp = Response::new(Body::from(serde_json::to_string(&TicketCreated { id: id.get() }).
//unwrap()));
//    *resp.status_mut() = StatusCode::CREATED;
//    Ok(resp)
//}

// Service‐level tests example:

#[tokio::test]
async fn service_create_ticket_works() {
    let mut repo = InMemoryTicketRepo::default();
    let draft = TicketDraft {
        title: TicketTitle::try_from("T".into()).unwrap(),
        description: TicketDescription::try_from("D".into()).unwrap(),
    };
    let id = create_ticket(&mut repo, draft.clone());
    assert_eq!(
        repo.get(id).unwrap(),
        Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::Open
        }
    );
}

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use ticket_fields::{TicketDescription, TicketTitle};
use tokio::sync::Mutex;
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

pub trait TicketRepo {
    fn add(&mut self, draft: TicketDraft) -> TicketId;
    fn update(&mut self, id: TicketId, patch: TicketPatch) -> Option<Ticket>;
    fn read(&mut self, id: TicketId) -> Option<Ticket>;
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
            let mut ticket = ticket.lock().await;
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
        let ticket = ticket.lock().await;
        let question_id = params.get("question").unwrap();
        let ticket_id = TicketId::set(question_id.parse::<u64>().unwrap());
        let ticket = ticket.get(ticket_id).unwrap();
        let created = serde_json::to_string(ticket).unwrap();
        let mut read = Response::new(Body::from(created));
        *read.status_mut() = StatusCode::OK;
        Ok(read)
    } else {
        let mut bad_request = Response::new(Body::from("Not Found"));
        *bad_request.status_mut() = StatusCode::BAD_REQUEST;
        Ok(bad_request)
    }
}
async fn patch(
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
        let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        let body: TicketPatch =
            serde_json::from_slice::<TicketPatch>(&body).expect("Parsing failed, validate");
        let question_id = params.get("question").unwrap();
        let mut ticket = ticket.lock().await;
        let ticket_id = TicketId::set(question_id.parse::<u64>().unwrap());
        let ticket = ticket.get_mut(ticket_id).unwrap();
        if let Some(items) = body.common {
            ticket.title = TicketTitle::try_from(items.title).unwrap();
            ticket.description = TicketDescription::try_from(items.description).unwrap();
        }
        if let Some(items) = body.extra_field {
            ticket.status = items;
        }
        let body = serde_json::to_string(&ticket).unwrap();
        let mut read = Response::new(Body::from(body));
        *read.status_mut() = StatusCode::OK;
        Ok(read)
    } else {
        let mut bad_request = Response::new(Body::from("Not Found"));
        *bad_request.status_mut() = StatusCode::BAD_REQUEST;
        Ok(bad_request)
    }
}
async fn router(
    req: Request<Body>,
    ticket: Arc<Mutex<TicketStore>>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/create") => create(req, ticket.clone()).await,
        (&Method::GET, "/read") => read(req, ticket.clone()).await,
        (&Method::PATCH, "/update") => patch(req, ticket.clone()).await,
        _ => {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
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

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Body, Request};

    // TODO rearchitect buisness logic from HTTP handling
    #[tokio::test]
    async fn test_create_ticket() {
        let store = Arc::new(Mutex::new(TicketStore::new()));
        let req = Request::builder()
            .method("POST")
            .uri("/create")
            .body(Body::from(r#"{"title":"Test","description":"Desc"}"#))
            .unwrap();
        let resp = create(req, store).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
