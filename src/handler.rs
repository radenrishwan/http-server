use tokio::{net::TcpStream, io::AsyncReadExt};

use crate::{request::Request, response::Response};

pub async fn handler(stream: &mut TcpStream, handler: &dyn Fn(Request, Response) -> ()) {
    // read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

   let request = Request::new(String::from_utf8_lossy(&buffer).to_string());
   let response = Response::new(
       "200 OK".to_string(),
              vec![],
              "".to_string()
   );

   handler(request, response);
}