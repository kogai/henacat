use chrono::offset::local::Local;

use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str) -> Self {
        Server { listener: TcpListener::bind(host).unwrap() }
    }

    pub fn listen(&mut self) {
        println!("[SERVER]: Waiting for connection from client...");

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    spawn(move || {
                        println!("[SERVER]: Recieve connection from client. {:?}", stream);
                        let request_headers = extract_head(&stream);
                        let response = create_response();
                        println!("{}", request_headers); // TODO: リクエストからパスを引き出す
                        stream.write_all(response.as_bytes()).unwrap();
                    });
                }
                Err(e) => println!("{:?}", e)
            }
        }
    }
}

fn extract_head(stream: &TcpStream) -> String {
    let mut buffered_stream = BufReader::new(stream);
    let mut recieve_buffer = String::new();

    loop {
        match buffered_stream.read_line(&mut recieve_buffer) {
            Ok(s) => {
                if s == 2 {
                    break; // Request header is end.
                }
            },
            Err(e) => {
                println!("read line failed... {:?}", e);
                break;
            },
        };
    }
    recieve_buffer
}

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

#[derive(Debug, PartialEq)]
pub enum Protcol {
    HTTP_1,
    HTTP_1_1,
    HTTP_2,
}

impl Protcol {
    pub fn from_string(x: &str) -> Self {
        match x {
            "HTTP/1" => Protcol::HTTP_1,
            "HTTP/1.1" => Protcol::HTTP_1_1,
            _ => Protcol::HTTP_2,
        }
    }
}

#[derive(Debug)]
pub struct Headers {
    method: Method,
    protcol: Protcol,
    uri: String,
}

fn extract_resource(h: String) -> Headers {
    let mut headers = h.lines();
    let mut request_line = headers.nth(0).unwrap().split(" ");
    let method = request_line.next().unwrap();
    let request_uri = request_line.next().unwrap();
    let protcol = request_line.next().unwrap();

    Headers {
        method: Method::from_string(method),
        uri: request_uri.to_string(),
        protcol: Protcol::from_string(protcol),
    }
}

fn create_response() -> String {
    let send_buffer = [
        format!("HTTP/1.1 200 OK"),
        format!("Date:  {}", Local::now()),
        format!("Server: Modoki/0.1"),
        format!("Connection: close"),
        format!("Content-type: text/html"),
        format!(""),
        format!(r#"
            <!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML 2.0//EN">
            <html><head>
            <title>OK</title>
            <link rel="stylesheet" type="text/css" href="/style.css">
            </head><body>
            <h1>It works!</h1>
            </body></html>
        "#),
    ].join("\r\n");

    send_buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_extract_resource() {
        let resource = extract_resource(r#"GET /favicon.ico HTTP/1.1
            Host: localhost:8000"#.to_string());
        assert_eq!(resource.method, Method::GET);
        assert_eq!(resource.uri, "/favicon.ico".to_string());
        assert_eq!(resource.protcol, Protcol::HTTP_1_1);
    }
}