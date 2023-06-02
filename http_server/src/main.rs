mod http_request;
mod http_response;

use crate::http_request::HttpRequest;
use std::str;
use std::{io::Read, net::TcpListener};

fn main() {
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
    }
}
