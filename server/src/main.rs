mod utils;

use std::net::TcpListener;

use crate::utils::{request::Request, response::Response};

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
