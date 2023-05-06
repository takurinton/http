use std::{collections::HashMap, io::Write, net::TcpStream};

use super::{method::Method, request::Request, status::HttpStatus};

pub struct Response {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            status: HttpStatus::OK,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn set_response(&mut self, request: &mut Request) {
        self.body = format!("Hello, path: {}", request.path);
        self.headers
            .insert(String::from("Content-Type"), String::from("text/plain"));
        self.headers
            .insert(String::from("Content-Length"), self.body.len().to_string());
        if Method::is_preflight(&request.method) {
            self.headers.insert(
                String::from("Access-Control-Request-Method"),
                String::from("*"),
            );
            self.headers.insert(
                String::from("Access-Control-Request-Headers"),
                String::from("*"),
            );
            self.headers.insert(
                String::from("Access-Control-Allow-Origin"),
                String::from("*"),
            );
            self.headers.insert(
                String::from("Access-Control-Max-Age"),
                String::from("86400"),
            );
        }

        self.status = Method::to_status(&request.method);
    }

    pub fn write(&self, stream: &mut TcpStream) {
        let response = self.format();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn format(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.get_code(),
            self.status.get_message()
        ));
        // dummy
        s.push_str("Date: Fri, 31 Dec 1999 23:59:59 GMT\r");
        s.push_str("Server: Rust Server\r\n");
        s.push_str("Connection: close\r\n");
        for (key, value) in &self.headers {
            s.push_str(&format!("{}: {}\r\n", key, value));
        }
        s.push_str("\r\n");
        s.push_str(&self.body);

        s
    }

    pub fn _log(&self) {
        println!("Status: {:?}", self.status);
        println!("Headers: {:?}", self.headers);
        println!("Body: {}", self.body);
    }
}
