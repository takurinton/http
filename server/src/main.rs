use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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

struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    fn new() -> Response {
        Response {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    fn format(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("HTTP/1.1 {} OK\r\n", self.status));
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
}

struct Server {
    request: Request,
    response: Response,
}

impl Server {
    fn new() -> Server {
        Server {
            request: Request::new(),
            response: Response::new(),
        }
    }

    fn run(&mut self) {
        let listener = TcpListener::bind("0.0.0.0:65535").unwrap();
        println!("Server listening on port 65535");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // request
                    self.request = Request::new();
                    self.request.parse(&mut stream);

                    // response
                    self.response = Response::new();
                    self.response.body = format!("Hello, {}", self.request.path);
                    self.response
                        .headers
                        .insert(String::from("Content-Type"), String::from("text/plain"));
                    self.response.headers.insert(
                        String::from("Content-Length"),
                        self.response.body.len().to_string(),
                    );

                    // write
                    let response = self.response.format();
                    println!("{}", response);
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        drop(listener);
    }
}

fn main() {
    let mut server = Server::new();
    server.run();
}
