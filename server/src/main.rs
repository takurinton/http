use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Copy, Clone, PartialEq)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    // HEAD,
    // CONNECT,
    OPTIONS,
    // TRACE,
    PATCH,
}

enum Status {
    Ok = 200,
    Created = 201,
    NoContent = 204,
}

struct HttpStatus;

impl HttpStatus {
    fn get_status(status: Status) -> (u8, String) {
        match status {
            Status::Ok => (200, String::from("OK")),
            Status::Created => (201, String::from("Created")),
            Status::NoContent => (204, String::from("No Content"))
        }
    }
}

struct Request {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new() -> Request {
        Request {
            method: Method::GET, // TODO
            path: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    fn parse(&mut self, stream: &mut TcpStream) {
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
        let method = match method.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            "OPTIONS" => Method::OPTIONS,
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

        let body = &request[headers_end + 4..];
        self.body = body.to_string();
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

    fn _log(&self) {
        let method = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
        };
        println!("Method: {:?}", method);
        println!("Path: {}", self.path);
        println!("Headers: {:?}", self.headers);
        println!("Body: {}", self.body);
    }
}

struct Response {
    status: (u8, String),
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    fn new() -> Response {
        Response {
            status: (200, String::from("OK")),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    // TODO: もう少し丁寧に。前から順に読んでいく。
    fn set_response(&mut self, request: &mut Request) {
        self.body = format!("Hello, path: {}", request.path);
        self.headers
            .insert(String::from("Content-Type"), String::from("text/plain"));
        self.headers
            .insert(String::from("Content-Length"), self.body.len().to_string());
        // TODO: 引数
        if request.method == Method::OPTIONS {
            self.headers
                .insert(String::from("Access-Control-Request-Method"), String::from("*"));
            self.headers
                .insert(String::from("Access-Control-Request-Headers"), String::from("*"));
            self.headers
                .insert(String::from("Access-Control-Allow-Origin"), String::from("*"));
            self.headers
                .insert(String::from("Access-Control-Max-Age"), String::from("86400"));
        }

        let status = match request.method {
            Method::GET => Status::Ok,
            Method::POST => Status::Created,
            Method::PUT => Status::Created,
            Method::PATCH => Status::Created,
            Method::DELETE => Status::NoContent,
            Method::OPTIONS => Status::NoContent,
        };

        let http_status = HttpStatus::get_status(status);
        self.status = http_status;
    }

    fn write(&self, stream: &mut TcpStream) {
        let response = self.format();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn _log(&self) {
        println!("Status: {:?}", self.status);
        println!("Headers: {:?}", self.headers);
        println!("Body: {}", self.body);
    }

    fn format(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("HTTP/1.1 {} {}\r\n", self.status.0, self.status.1));
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
}

struct Server {
    listener: TcpListener,
    request: Request,
    response: Response,
}

impl Server {
    fn new() -> Server {
        Server {
            listener: TcpListener::bind("0.0.0.0:65535").unwrap(),
            request: Request::new(),
            response: Response::new(),
        }
    }

    fn run(&mut self) {
        println!("Server listening on port 65535");
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // request
                    self.request = Request::new();
                    self.request.parse(&mut stream);

                    // response
                    self.response = Response::new();
                    self.response.set_response(&mut self.request);
                    self.response.write(&mut stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

fn main() {
    let mut server = Server::new();
    server.run();
}
