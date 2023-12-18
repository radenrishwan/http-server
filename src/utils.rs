#[macro_use]
pub mod cookie {
    pub fn set_cookie(key: &str, value: &str, path : &str, domain : &str, exp : &str) -> String {
        format!("Set-Cookio: {}={}; expires={}, path={}, domain={}", key, value, exp, path, domain)
   }
}