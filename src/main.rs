mod handler;
mod request;
mod response;
mod utils;

use tokio::{io::AsyncReadExt, net::TcpListener};

use crate::handler::file_handler;
use crate::request::Request;
use crate::response::Response;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        println!("Connection from: {}", stream.peer_addr().unwrap());

        tokio::spawn(async move {
            // get stream
            let stream = &mut stream;

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).await.unwrap();

            let request = Request::new(String::from_utf8_lossy(&buffer).to_string());

            if request.path == "/" {
                let response = Response::new(
                    "200 OK".to_string(),
                    vec![],
                    "<h1>Index</h1>".to_string(),
                );

                // send response
                response.send(stream).await.unwrap();
            }

            if request.path == "/hello" {
                let response = Response::new(
                    "200 OK".to_string(),
                    vec![],
                    "<h1>Hello, World!</h1>".to_string(),
                );

                // send response
                response.send(stream).await.unwrap();
            }

            if request.path == "/serve" {
                file_handler(stream, "static/somefile.html", "text/html").await.unwrap();
                
            }
        });
    }
}