use std::collections::HashMap;

pub enum request_method {
    GET,
    POST,
}

enum http_version {
    V1,
    V2,
}

struct request_line {
    method: request_method,
    resource: String,
    version: http_version,
}

enum request_body {
    Conteng(String),
    NONE,
}

pub struct http_request {
    request_line: request_line,
    request_header: HashMap<String, String>,
    request_body: request_body,
}

impl From<&str> for request_method {
    fn from(value: &str) -> Self {
        if value == "GET" {
            return request_method::GET;
        } else {
            return request_method::POST;
        }
    }
}
