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

fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let val = std::str::from_utf8(&buffer[..]).unwrap();
    let query = val.split_terminator("\r\n").collect::<Vec<&str>>();
    if let Some(first) = query.first() {
        let collection: Vec<&str> = first.split_whitespace().collect();
        if let Some(url) = collection.iter().nth(1) {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}!",
                url.len(),
                url
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
    stream.flush().unwrap();
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let _ = handle_client(&mut stream?);
    }
    Ok(())
}
