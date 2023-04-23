use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Copy, Clone)]
enum Method {
    GET,
    // POST,
    // PUT,
    // DELETE,
    // HEAD,
    // CONNECT,
    // OPTIONS,
    // TRACE,
    // PATCH,
}

struct Request {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
}

impl Request {
    fn new() -> Request {
        Request {
            method: Method::GET, // TODO
            path: String::new(),
            headers: HashMap::new(),
        }
    }

    fn parse(&mut self, stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = std::str::from_utf8(&buffer).unwrap();

        let request_line_end = request.find("\r\n").unwrap();
        let request_line = &request[..request_line_end];
        let mut request_parts = request_line.split_whitespace();

        let method = String::from(request_parts.next().unwrap());
        let method = match method.as_str() {
            "GET" => Method::GET,
            _ => panic!("Invalid HTTP method"),
        };
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
    }

    // fn parse(&mut self, data: TcpStream) -> Self {
    //     let mut buf = [0 as u8; 1024];
    //     let mut stream = data;

    //     stream.read(&mut buf).unwrap();
    //     let mut i = 0;
    //     let method = Request::get_method(self, &mut i, &buf);
    //     let path = Request::get_path(self, &mut i, &buf);

    //     return Request {
    //         method,
    //         path,
    //         // TODO
    //         headers: HashMap::new(),
    //     };
    // }

    // fn get_method(&mut self, i: &mut usize, buf: &[u8]) -> Method {
    //     let mut index = i.clone();
    //     let mut method = String::new();
    //     while buf[index] != b' ' {
    //         method.push(buf[index] as char);
    //         index += 1;
    //     }

    //     // どこまで進めたかを書き換える
    //     *i = index + 1;

    //     let m = match method.as_str() {
    //         "GET" => Method::GET,
    //         _ => panic!("Invalid HTTP method"),
    //     };

    //     *i = index + 1;

    //     self.method = m;
    //     m
    // }

    // fn get_path(&mut self, i: &mut usize, buf: &[u8]) -> String {
    //     let mut index = i.clone();
    //     let mut path = String::new();

    //     while buf[index] != b' ' {
    //         path.push(buf[index] as char);
    //         index += 1;
    //     }

    //     self.path = path.clone();
    //     path
    // }

    fn log(&self) {
        let method = match self.method {
            Method::GET => "GET",
        };
        println!("Method: {:?}", method);
        println!("Path: {}", self.path);
        println!("Headers: {:?}", self.headers);
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:65535").unwrap();
    println!("Server listening on port 65535");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut request = Request::new();
                request.parse(&mut stream);
                request.log();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
