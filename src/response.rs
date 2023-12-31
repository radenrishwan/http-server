use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug, Clone)]
pub struct Response {
    pub status: String,
    pub headers: Vec<String>,
    pub body: String
}

impl Response {
    /// Create a new response
    /// by default, contents length, and content type will be set
    /// to the header
    pub fn new(status: String, headers: Vec<String>, body: String) -> Response {
        let mut headers = headers;
        headers.push(format!("Content-Length: {}; charset=utf-8", body.len()));

        // check if the content type is already set
        if !headers.contains(&format!("Content-Type: {}", "text/html")) {
            headers.push(format!("Content-Type: {}", "text/html"));
        }

        // check if status code is already set
        if !headers.contains(&format!("Status: {}", status)) {
            headers.push(format!("Status: {}", status));
        }
        
        Response {
            status: status,
            headers: headers,
            body: body
        }
    }

    /// Send the response to the stream.
    /// currently only supports http/1.1
    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let response = format!("HTTP/1.1 {}\r\n{}\r\n\r\n{}", self.status, self.headers.join("\r\n"), self.body);

        stream.write_all(response.as_bytes()).await
    }
}