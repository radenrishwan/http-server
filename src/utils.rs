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

    pub fn get(headers: Vec<String>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        // split cookies
        for header in headers {
            if header.contains("Cookie:") {
                let cookie = header.split("Cookie:").collect::<Vec<&str>>()[1];
                let cookie = cookie.split(";").collect::<Vec<&str>>();

                for c in cookie {
                    let cookie = c.split("=").collect::<Vec<&str>>();
                    result.insert(cookie[0].to_string(), cookie[1].to_string());
                }

                break;
            }
        }

        result
    }

    pub fn get_value(headers: Vec<String>, key: &str) -> Result<String, Error> {
        // get cookie
        let cookies = get(headers);

        if cookies.contains_key(key) {
            return Ok(cookies.get(key).unwrap().to_string());
        }

        Err(Error::new(ErrorKind::NotFound, "Cookie not found"))
    }
}
