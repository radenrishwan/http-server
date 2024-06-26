use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    /// parse request
    /// # Examples
    /// ```
    /// let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    ///
    /// loop {
    ///     let (mut stream, _) = listener.accept().await.unwrap();
    ///     println!("Connection from: {}", stream.peer_addr().unwrap());
    ///
    ///    tokio::spawn(async move {
    ///         // read request
    ///         let stream = &mut stream;
    ///
    ///         let mut buffer = [0; 1024];
    ///         stream.read(&mut buffer).await.unwrap();
    ///
    ///         let request = Request::new(String::from_utf8_lossy(&buffer).to_string());
    ///    });
    /// }
    /// ```
    pub fn new(request: String) -> Request {
        let split = request.split(" ").collect::<Vec<&str>>();

        let method = split[0];
        let path = split[1];
        let version = split[2].split("\r\n").collect::<Vec<&str>>()[0];

        let headers = Request::parse_header(
            request
                .lines()
                .filter(|x| x.contains(":"))
                .map(|x| x.to_string())
                .collect(),
        );

        let body = request.split("\r\n\r\n").collect::<Vec<&str>>()[1];

        Request {
            method: method.to_string(),
            path: path.to_string(),
            version: version.to_string(),
            headers: headers,
            body: String::from(body),
        }
    }

    fn parse_header(headers: Vec<String>) -> HashMap<String, String> {
        let mut result = HashMap::new();

        for header in headers {
            let header = header.split(":").collect::<Vec<&str>>();
            result.insert(header[0].to_string(), header[1].trim().to_string());
        }

        result
    }
}
