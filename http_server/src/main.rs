mod http_request;
mod http_response;

use crate::http_request::HttpRequest;
use crate::http_request::HttpVersion;
use crate::http_response::HttpResponse;
use crate::http_response::ResponseBody;
use crate::http_response::ResponseLine;
use std::collections::HashMap;
use std::str;
use std::{io::Read, io::Write, net::TcpListener};

fn main() {
    let mut header = HashMap::new();
    header.insert("Content-type".to_string(), "text/html".to_string());
    let response = HttpResponse {
        response_line: ResponseLine {
            http_version: HttpVersion::V1,
            status_code: String::from("200"),
            status_message: String::from("OK"),
        },
        response_header: header,
        response_body: ResponseBody::None,
    };

    let response_str = String::from(&response);
    println!("{}", response_str);

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Running on port 8080");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let str = str::from_utf8(&buffer).unwrap();
        let http_request = HttpRequest::from(str);
        println!("{:#?}", http_request);

        // return response
        stream.write(response_str.as_bytes()).unwrap();
    }
}
