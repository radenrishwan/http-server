use tokio::{fs::File, io::AsyncReadExt, net::TcpStream};

use crate::{request::Request, response::Response};

pub async fn handler(stream: &mut TcpStream, handler: &dyn Fn(Request, Response) -> ()) {
    // read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let request = Request::new(String::from_utf8_lossy(&buffer).to_string());
    let response = Response::new("200 OK".to_string(), vec![], "".to_string());

    handler(request, response);
}

pub async fn file_handler(stream: &mut TcpStream, path: &str) -> Result<(), std::io::Error> {
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
        vec![],
        String::from_utf8_lossy(&buffer).to_string(),
    );

    response.send(stream).await.unwrap();

    // send into stream
    Ok(())
}
