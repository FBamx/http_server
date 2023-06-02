mod http_request;
mod http_response;

use crate::http_request::{HttpRequest, RequestLine};

fn main() {
    let request = HttpRequest::from("GET /login HTTP/1.1\r\nContent-type:html\r\n\r\nnihao");
    println!("{:#?}", request);
}
