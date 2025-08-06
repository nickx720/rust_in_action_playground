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
};

use nyquest::{blocking::Request, header::FORWARDED};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

//fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
//    let mut buffer = [0; 1024];
//    let _ = stream.read(&mut buffer);
//    let val = std::str::from_utf8(&buffer[..]).unwrap();
//    let query = val.split_terminator("\r\n").collect::<Vec<&str>>();
//    if let Some(first) = query.first() {
//        let collection: Vec<&str> = first.split_whitespace().collect();
//        if let Some(url) = collection.iter().nth(1) {
//            let client = nyquest::ClientBuilder::default()
//                .user_agent("curl/8.7.1 nyquest/0")
//                .build_blocking()
//                .expect("Failed to build client");
//            let response_from_client = client
//                .request(
//                    Request::get(url.to_string())
//                        // TODO get the value dynamically
//                        .with_header(FORWARDED, "for=127.0.0.1".to_string()),
//                )
//                .expect("Failed to get response");
//
//            let content = &response_from_client.bytes().unwrap();
//            let mut response_buf: Vec<u8> = Vec::new();
//            response_buf.extend_from_slice(b"HTTP/1.1 200 OK\r\n");
//            response_buf
//                .extend_from_slice(format!("Content-Length: {}\r\n\r\n", content.len()).as_bytes());
//            response_buf.extend_from_slice(content);
//            let _ = stream.write_all(&response_buf);
//        }
//    }
//    stream.flush().unwrap();
//    Ok(())
//}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //    nyquest_preset::register();
    //    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //
    //    // accept connections and process them serially
    //    for stream in listener.incoming() {
    //        let _ = handle_client(&mut stream?);
    //    }
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
    Ok(())
}
