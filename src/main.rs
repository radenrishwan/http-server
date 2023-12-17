mod request;

use tokio::{
    net::TcpListener, io::{AsyncWriteExt, AsyncReadExt},
};

use crate::request::Request;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        println!("Connection from: {}", stream.peer_addr().unwrap());

       tokio::spawn(async move {
            // read request
            let stream = &mut stream;

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).await.unwrap();

            println!("Request: {}", String::from_utf8_lossy(&buffer));

            let req = Request::new(String::from_utf8_lossy(&buffer).to_string());

           // write a response
           let content = format!("{:?}", req);
           let header = write_header(content.len());
           let response = header + content.as_str();

           stream.write_all(response.as_bytes()).await.unwrap();
       });
    }
}

fn write_header(content: usize) -> String {
    let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", content);

    header
}