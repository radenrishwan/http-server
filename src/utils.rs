#[macro_use]
pub mod cookie {
    use std::io::{Error, ErrorKind};
    use crate::response::Response;

    pub fn set(key: &str, value: &str, path : &str, domain : &str, exp : &str) -> String {
        format!("Set-Cookio: {}={}; expires={}, path={}, domain={}", key, value, exp, path, domain)
   }

    pub fn get(response: Response, key: &str) -> Result<&str, Error> {
        return match response.headers.get("Cookie") {
            Ok(cookies) => {
                let cookies = cookies.split(";").collect::<Vec<&str>>();
                for cookie in cookies {
                    let cookie = cookie.split("=").collect::<Vec<&str>>();
                    if cookie[0] == key {
                        return Ok(cookie[1]);
                    }
                }

                Ok("")
            },
            Err(_) => {
                Err(Error::new(ErrorKind::NotFound, "Cookie not found"))
            },
        };


    }
}