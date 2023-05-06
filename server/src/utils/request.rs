use std::{collections::HashMap, io::Read, net::TcpStream};

use super::method::Method;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: Method::GET, // TODO
            path: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn parse(&mut self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = match std::str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let request_line_end = request.find("\r\n").unwrap();
        let request_line = &request[..request_line_end];
        let mut request_parts = request_line.split_whitespace();

        let method = String::from(request_parts.next().unwrap());
        let method = Method::from_str(&method);
        let path = String::from(request_parts.next().unwrap());

        self.method = method;
        self.path = path;

        let headers_end = request.find("\r\n\r\n").unwrap();
        let headers = &request[request_line_end + 2..headers_end];
        for header in headers.lines() {
            let mut header_parts = header.split(": ");
            let header_name = header_parts.next().unwrap();
            let header_value = header_parts.next().unwrap();
            self.headers
                .insert(String::from(header_name), String::from(header_value));
        }

        let body = &request[headers_end + 4..];
        self.body = body.to_string();
    }

    pub fn _log(&self) {
        let method = self.method.to_str();
        println!("Method: {:?}", method);
        println!("Path: {}", self.path);
        println!("Headers: {:?}", self.headers);
        println!("Body: {}", self.body);
    }
}
