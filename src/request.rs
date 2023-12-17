#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: String
}

impl Request {
    pub fn new(request: String) -> Request {
        let split = request.split(" ").collect::<Vec<&str>>();

        let method = split[0];
        let path = split[1];
        let version = split[2].split("\r\n").collect::<Vec<&str>>()[0]; 

        Request {
            method: method.to_string(),
            path: path.to_string(),
            version: version.to_string(),
            headers: Vec::new(),
            body: String::new()
        }
    }
}