mod http_request;
mod http_response;

use crate::http_request::request_method;

fn main() {
    let _method = request_method::from(String::from("GET").as_str());
    match _method {
        request_method::GET => println!("GET"),
        request_method::POST => println!("POST"),
    }
}
