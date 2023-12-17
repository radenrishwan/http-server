#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<String>,
    pub body: String
}

impl Request {
    pub fn new(request: String) -> Request {
        let split = request.split(" ").collect::<Vec<&str>>();

        let method = split[0];
        let path = split[1];
        let version = split[2].split("\r\n").collect::<Vec<&str>>()[0];

        let headers: Vec<String> = request.lines()
            .filter(|x| x.contains(":"))
            .map(|x| x.to_string())
            .collect();

        let body = request.split("\r\n\r\n").collect::<Vec<&str>>()[1];

        Request {
            method: method.to_string(),
            path: path.to_string(),
            version: version.to_string(),
            headers: headers,
            body: String::from(body),
        }
    }
}