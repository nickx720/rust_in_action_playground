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
use serde::Deserialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use ticket_fields::{TicketDescription, TicketTitle};

use crate::ticket::{Ticket, TicketStore};
mod ticket;

#[derive(Deserialize, Debug)]
struct ParseBody {
    title: String,
    description: String,
}

async fn create(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body: ParseBody =
        serde_json::from_slice::<ParseBody>(&body).expect("Parsing failed, validate");
    let title = TicketTitle::try_from(body.title);
    let description = TicketTitle::try_from(body.description);
    let mut created = Response::new(Body::from("Created"));
    *created.status_mut() = StatusCode::CREATED;
    Ok(created)
}
async fn read(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Read stub")))
}
async fn patch(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Patch stub")))
}
async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/create") => create(req).await,
        (&Method::GET, "/read") => read(req).await,
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
    let mut ticket = TicketStore::new();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
}
