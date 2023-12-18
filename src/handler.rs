use tokio::{fs::File, io::AsyncReadExt, net::TcpStream};

use crate::{request::Request, response::Response};

pub async fn handler(stream: &mut TcpStream, handler: &dyn Fn(Request, Response) -> ()) -> Result<(), std::io::Error>{
    // read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let request = Request::new(String::from_utf8_lossy(&buffer).to_string());
    let response = Response::new("200 OK".to_string(), vec![], "".to_string());

    handler(request, response);

    Ok(())
}

/// serve a file
/// 
/// example:
/// ```
/// let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
///
/// loop {
///     let (mut stream, _) = listener.accept().await.unwrap();
///     println!("Connection from: {}", stream.peer_addr().unwrap());
///
///    tokio::spawn(async move {
///         let stream = &mut stream;
///
///         file_handler(stream, "static/somefile.html").await.unwrap();
///    });
/// }
/// ```
pub async fn file_handler(stream: &mut TcpStream, path: &str, file_type: &str) -> Result<(), std::io::Error> {
    // read file
    let mut buffer = [0; 1024];
    match File::open(path).await {
        Ok(mut file) => {
            file.read(&mut buffer).await.unwrap();
        }
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));
        }
    };

    let response = Response::new(
        "200 OK".to_string(),
        vec![
            format!("Content-Type: {}", file_type),
        ],
        String::from_utf8_lossy(&buffer).to_string(),
    );

    response.send(stream).await.unwrap();

    // send into stream
    Ok(())
}