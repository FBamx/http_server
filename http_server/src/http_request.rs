use std::collections::HashMap;

#[derive(Debug)]
enum RequestMethod {
    GET,
    POST,
}

#[derive(Debug)]
enum HttpVersion {
    V1,
    V2,
}

#[derive(Debug)]
pub struct RequestLine {
    method: RequestMethod,
    resource: String,
    version: HttpVersion,
}

#[derive(Debug)]
enum RequestBody {
    Content(String),
    NONE,
}

#[derive(Debug)]
pub struct HttpRequest {
    request_line: RequestLine,
    request_header: HashMap<String, String>,
    request_body: RequestBody,
}

impl From<&str> for RequestMethod {
    fn from(method: &str) -> Self {
        if method == "GET" {
            return RequestMethod::GET;
        } else {
            return RequestMethod::POST;
        }
    }
}

impl From<&str> for HttpVersion {
    fn from(version: &str) -> Self {
        if version == "HTTP/1.1" {
            return HttpVersion::V1;
        } else {
            return HttpVersion::V2;
        }
    }
}

impl RequestLine {
    fn new() -> Self {
        return RequestLine {
            method: RequestMethod::GET,
            resource: String::from("/"),
            version: HttpVersion::V1,
        };
    }
}

impl From<&str> for RequestLine {
    fn from(request_line: &str) -> Self {
        let mut word = request_line.split_whitespace();
        let method = RequestMethod::from(word.next().unwrap());
        let resource = word.next().unwrap();
        let version = HttpVersion::from(word.next().unwrap());
        return RequestLine {
            method: method,
            resource: resource.to_string(),
            version: version,
        };
    }
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string()
    }
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }

    (key, value)
}

impl From<&str> for HttpRequest {
    fn from(http_request: &str) -> Self {
        let mut request_line = RequestLine::new();
        let mut request_header = HashMap::new();
        let mut request_body = RequestBody::NONE;
        for line in http_request.lines() {
            if line.contains("HTTP") {
                request_line = RequestLine::from(line);
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                request_header.insert(key, value);
            } else {
                request_body = RequestBody::Content(line.to_string());
            }
        }
        HttpRequest {
            request_line: request_line,
            request_header: request_header,
            request_body: request_body,
        }
    }
}
