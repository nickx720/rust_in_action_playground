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
//
//    Sending the response to the client.

// TODO use tokio
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    let _ = stream.write_all(response);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
