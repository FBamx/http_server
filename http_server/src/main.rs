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
    let reponse_line = ResponseLine::new(HttpVersion::V1, String::from("200"), String::from("OK"));
    let mut response_header = HashMap::new();
    response_header.insert("Content-type".to_string(), "text/html".to_string());
    response_header.insert("kkk".to_string(), "vv".to_string());

    let mut file = std::fs::File::open("./http_server/static/index.html").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let response_body = ResponseBody::Content(content);
    let response = HttpResponse::new(reponse_line, response_header, response_body);

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
        let response_str = String::from(&response);
        println!("{}", response_str);
        stream.write(response_str.as_bytes()).unwrap();
    }
}
