//     Have your proxy server start up and listen for connections. In my case I decided to listen on port 8989.

//    When a request is received parse it to extract the target host.
//
//    Create a new socket / connection to the target server.
//
//    Forward the request, minus the hop by hop headers.
//
//    Change the GET request.
//
//    Add the ‘X-Forwarded-For’ header.
//
//    Read the response from the target server and set the correct response headers before,
//    Sending the response to the client.

// TODO use tokio
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use nyquest::{blocking::Request, header::FORWARDED};

fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let val = std::str::from_utf8(&buffer[..]).unwrap();
    let query = val.split_terminator("\r\n").collect::<Vec<&str>>();
    if let Some(first) = query.first() {
        let collection: Vec<&str> = first.split_whitespace().collect();
        if let Some(url) = collection.iter().nth(1) {
            let client = nyquest::ClientBuilder::default()
                .user_agent("curl/8.7.1 nyquest/0")
                .build_blocking()
                .expect("Failed to build client");
            let response_from_client = client
                .request(
                    Request::get(url.to_string())
                        // TODO get the value dynamically
                        .with_header(FORWARDED, "for=127.0.0.1".to_string()),
                )
                .expect("Failed to get response");
            let response = response_from_client
                .bytes()
                .expect("Unable to send response");
            let _ = stream.write_all(&response);
        }
    }
    stream.flush().unwrap();
    Ok(())
}

fn main() -> std::io::Result<()> {
    nyquest_preset::register();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let _ = handle_client(&mut stream?);
    }
    Ok(())
}
