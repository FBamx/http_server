use std::collections::HashMap;

use crate::http_request::HttpVersion;

#[derive(Debug)]
pub struct ResponseLine {
    http_version: HttpVersion,
    status_code: String,
    status_message: String,
}

#[derive(Debug)]
pub enum ResponseBody {
    Content(String),
    None,
}

#[derive(Debug)]
pub struct HttpResponse {
    response_line: ResponseLine,
    response_header: HashMap<String, String>,
    response_body: ResponseBody,
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
    pub fn new(http_version: HttpVersion, status_code: String, status_message: String) -> Self {
        ResponseLine {
            http_version,
            status_code,
            status_message,
        }
    }
}

impl From<&HttpResponse> for String {
    fn from(http_response: &HttpResponse) -> String {
        let response_line = String::from(&http_response.response_line);
        let mut headers = String::from("");
        for (k, v) in &http_response.response_header {
            let header = format!("{}:{}\r\n", k, v);
            headers = format!("{}{}", headers, header);
        }
        let response_body = match &http_response.response_body {
            ResponseBody::Content(content) => String::from(content),
            ResponseBody::None => String::from(""),
        };
        let response = format!("{}\r\n{}\r\n{}", response_line, headers, response_body);
        response
    }
}

impl HttpResponse {
    pub fn new(
        response_line: ResponseLine,
        response_header: HashMap<String, String>,
        response_body: ResponseBody,
    ) -> Self {
        HttpResponse {
            response_line,
            response_header,
            response_body,
        }
    }
}
