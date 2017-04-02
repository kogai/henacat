#[derive(Debug, PartialEq)]
pub enum Method {
    OPTIONS,
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

impl Method {
    pub fn from_string(x: &str) -> Self {
        match x {
            "OPTIONS" => Method::OPTIONS,
            "HEAD" => Method::HEAD,
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "TRACE" => Method::TRACE,
            "CONNECT" => Method::CONNECT,
            _ => Method::GET,
        }
    }
}
