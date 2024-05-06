pub mod cookie {
    use chrono::{DateTime, Utc};

    use std::{
        collections::HashMap,
        io::{Error, ErrorKind},
    };

    /// set cookie
    ///
    /// examples:
    /// ```
    /// let current = Utc::now() + Duration::days(7);
    ///
    /// let c = cookie::set("foo", "bar", "/cookie", ".localhost", formatted_time.as_str());
    /// let response = Response::new(
    ///     "200 OK".to_string(),
    ///     vec![
    ///         c
    ///     ],
    ///     body,
    /// );
    ///
    /// response.send(stream).await.unwrap();
    /// ```
    pub fn set(key: &str, value: &str, path: &str, domain: &str, exp: DateTime<Utc>) -> String {
        let formatted_time = exp.format("%a, %e %b %Y %H:%M:%S GMT").to_string();
        if domain == "" {
            return format!(
                "Set-Cookie: {}={}; path={}; expires={}; Secure; SameSite=None",
                key, value, path, formatted_time
            );
        }

        format!(
            "Set-Cookie: {}={}; path={}; expires={}; domain={}; Secure; SameSite=None",
            key, value, path, formatted_time, domain
        )
    }

    pub fn get(headers: &HashMap<String, String>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        // split cookies
        let headers = headers
            .get("Cookie")
            .unwrap()
            .split(";")
            .collect::<Vec<&str>>();

        for header in headers {
            let header = header.split("=").collect::<Vec<&str>>();
            result.insert(
                header[0].to_string().trim_start().to_string(),
                header[1].to_string().to_string(),
            );
        }

        result
    }

    pub fn get_value(headers: &HashMap<String, String>, key: &str) -> Result<String, Error> {
        // get cookie
        let cookies = get(headers);

        if cookies.contains_key(key) {
            return Ok(cookies.get(key).unwrap().to_string());
        }

        Err(Error::new(ErrorKind::NotFound, "Cookie not found"))
    }
}

// GET /chat HTTP/1.1
// Host: server.example.com
// Upgrade: websocket
// Connection: Upgrade
// Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
// Origin: http://example.com
// Sec-WebSocket-Protocol: chat, superchat
// Sec-WebSocket-Version: 13

// HTTP/1.1 101 Switching Protocols
// Upgrade: websocket
// Connection: Upgrade
// Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=
// Sec-WebSocket-Protocol: chat

pub mod websocket {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use sha1::{Digest, Sha1};
    use tokio::{io::AsyncWriteExt, net::TcpStream};
    static WS_KEY: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

    pub async fn upgrade(socket: &mut TcpStream, id: &str) -> Result<(), std::io::Error> {
        let key = generate_key(id.to_string());

        let headers = format!("
            HTTP/1.1 101 Switching Protocols\r\n
            Upgrade: websocket\r\n
            Connection: Upgrade\r\n
            Sec-WebSocket-Accept: {}\r\n
            Sec-WebSocket-Protocol: chat\r\n",
            key
        );

        let headers = headers.replace(" ", "");
    
        print!("headers: {}", headers);

        match socket.write_all(headers.as_bytes()).await {
            Ok(_) => {
                println!("websocket upgraded");
            }
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "websocket upgrade failed",
                ));
            }
        }

        Ok(())
    }

    fn generate_key(id: String) -> String {
        let mut hash = Sha1::new();
        hash.update(id.as_bytes());
        hash.update(WS_KEY.as_bytes());

        STANDARD.encode(&hash.finalize())
    }
}
