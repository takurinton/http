use super::status::HttpStatus;

#[derive(Copy, Clone, PartialEq)]
pub enum Method {
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
    pub fn from_str(method: &str) -> Method {
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

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
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

    #[allow(dead_code)]
    pub fn has_body(&self) -> bool {
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

    #[allow(dead_code)]
    pub fn has_response_body(&self) -> bool {
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

    pub fn to_status(&self) -> HttpStatus {
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

    pub fn is_preflight(&self) -> bool {
        match self {
            Method::OPTIONS => true,
            _ => false,
        }
    }
}
