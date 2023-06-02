use std::collections::HashMap;

use crate::http_request::process_header_line;
use crate::http_request::HttpVersion;

#[derive(Debug)]
pub struct ResponseLine {
    pub http_version: HttpVersion,
    pub status_code: String,
    pub status_message: String,
}

#[derive(Debug)]
pub enum ResponseBody {
    Content(String),
    None,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub response_line: ResponseLine,
    pub response_header: HashMap<String, String>,
    pub response_body: ResponseBody,
}

impl From<&str> for ResponseLine {
    fn from(response_line: &str) -> Self {
        let mut word = response_line.split_whitespace();
        let version: HttpVersion = word.next().unwrap().into();
        let code = word.next().unwrap();
        let message = word.next().unwrap();

        ResponseLine {
            http_version: version,
            status_code: code.to_string(),
            status_message: message.to_string(),
        }
    }
}

impl From<&ResponseLine> for String {
    fn from(response_line: &ResponseLine) -> String {
        let version = match response_line.http_version {
            HttpVersion::V1 => String::from("HTTP/1.1 "),
            HttpVersion::V2 => String::from("HTTP/2.0 "),
        };
        let line = format!(
            "{} {} {}",
            version, response_line.status_code, response_line.status_message
        );
        line
    }
}

impl ResponseLine {
    fn new() -> Self {
        ResponseLine {
            http_version: HttpVersion::V1,
            status_code: String::from("200"),
            status_message: String::from("OK"),
        }
    }
}

impl From<&str> for HttpResponse {
    fn from(http_response: &str) -> Self {
        let mut response_line = ResponseLine::new();
        let mut response_header = HashMap::new();
        let mut response_body = ResponseBody::None;

        for line in http_response.lines() {
            if line.contains("HTTP") {
                response_line = ResponseLine::from(line);
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                response_header.insert(key, value);
            } else {
                response_body = ResponseBody::Content(line.to_string());
            }
        }

        HttpResponse {
            response_line,
            response_header,
            response_body,
        }
    }
}

impl From<&HttpResponse> for String {
    fn from(http_response: &HttpResponse) -> String {
        let response_line = String::from(&http_response.response_line);
        // WARN: just satifies one situation that only have one header
        let mut header = String::from("");
        for (k, v) in &http_response.response_header {
            header = format!("{}:{}\r\n", k, v);
        }
        let response = format!("{}\r\n{}\r\n<h1>hello world</h1>", response_line, header);
        response
    }
}
