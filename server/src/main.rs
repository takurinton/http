use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::num::NonZeroU16;

#[derive(Copy, Clone, PartialEq)]
enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    // HEAD,
    // CONNECT,
    // TRACE,
}

impl Method {
    fn from_str(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            "OPTIONS" => Method::OPTIONS,
            // "HEAD" => Method::HEAD,
            // "CONNECT" => Method::CONNECT,
            // "TRACE" => Method::TRACE,
            _ => panic!("Invalid HTTP method"),
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
            // Method::HEAD => "HEAD",
            // Method::CONNECT => "CONNECT",
            // Method::TRACE => "TRACE",
        }
    }

    fn has_body(&self) -> bool {
        match self {
            Method::GET => false,
            Method::POST => true,
            Method::PUT => true,
            Method::PATCH => true,
            Method::DELETE => false,
            Method::OPTIONS => false,
            // Method::HEAD => false,
            // Method::CONNECT => false,
            // Method::TRACE => false,
        }
    }

    fn has_response_body(&self) -> bool {
        match self {
            Method::GET => true,
            Method::POST => true,
            Method::PUT => true,
            Method::PATCH => true,
            Method::DELETE => false,
            Method::OPTIONS => true,
            // Method::HEAD => false,
            // Method::CONNECT => false,
            // Method::TRACE => false,
        }
    }

    fn to_status(&self) -> HttpStatus {
        match self {
            Method::GET => HttpStatus::OK,
            Method::POST => HttpStatus::CREATED,
            Method::PUT => HttpStatus::CREATED,
            Method::PATCH => HttpStatus::CREATED,
            Method::DELETE => HttpStatus::NO_CONTENT,
            Method::OPTIONS => HttpStatus::OK,
            // Method::HEAD => HttpStatus::OK,
            // Method::CONNECT => HttpStatus::OK,
            // Method::TRACE => HttpStatus::OK,
        }
    }

    fn is_preflight(&self) -> bool {
        match self {
            Method::OPTIONS => true,
            _ => false,
        }
    }
}

macro_rules! http_status {
    (
        $(
            $(#[$docs:meta])*
            ($constants:ident, $code:expr, $message:expr);
        )+
    ) => {
        impl HttpStatus {
        $(
            $(#[$docs])*
            const $constants: HttpStatus = HttpStatus(unsafe {
                NonZeroU16::new_unchecked($code)
            });
        )+

        fn get_code(&self) -> u16 {
            self.0.get()
        }

        fn get_message(&self) -> &'static str {
            match self.0.get() {
                $(
                    $code => $message,
                )+
                _ => panic!("Invalid HTTP status code"),
            }
        }
    }

    }
}

http_status! {
    (OK, 200, "OK");
    (CREATED, 201, "Created");
    (NO_CONTENT, 204, "No Content");
}

#[derive(Copy, Clone, Debug)]
struct HttpStatus(NonZeroU16);

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

    fn _log(&self) {
        let method = self.method.to_str();
        println!("Method: {:?}", method);
        println!("Path: {}", self.path);
        println!("Headers: {:?}", self.headers);
        println!("Body: {}", self.body);
    }
}

struct Response {
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    fn new() -> Response {
        Response {
            status: HttpStatus::OK,
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
