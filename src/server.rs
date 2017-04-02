use chrono::offset::local::Local;

use std::io::{Write, Read, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::fs::File;

use content_type::ContentType;
use method::Method;
use status::HttpStatus;

static ROOT_DIR: &str = "httpdocs";

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
                        let response = create_response(&request_headers);
                        stream.write_all(response.as_slice()).unwrap();
                    });
                }
                Err(e) => println!("{:?}", e)
            }
        }
    }
}

fn extract_head(stream: &TcpStream) -> Headers {
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
    extract_headers(recieve_buffer)
}

fn extract_headers(h: String) -> Headers {
    let mut headers = h.lines();
    let mut request_line = headers.nth(0).unwrap().split(" ");
    let method = request_line.next().unwrap();
    let request_uri = request_line.next().unwrap();
    let protcol = request_line.next().unwrap();

    Headers {
        method: Method::from_string(method),
        uri: request_uri.to_string(),
        protcol: Protcol::from_string(protcol),
        content_type: ContentType::from_path(request_uri),
    }
}

#[derive(Debug, PartialEq)]
pub enum Protcol {
    HTTP1,
    HTTP2,
}

impl Protcol {
    pub fn from_string(x: &str) -> Self {
        match x {
            "HTTP/1" => Protcol::HTTP1,
            "HTTP/1.1" => Protcol::HTTP1,
            _ => Protcol::HTTP2,
        }
    }
}

#[derive(Debug)]
pub struct Headers {
    method: Method,
    protcol: Protcol,
    uri: String,
    content_type: ContentType,
}

fn create_response(request_header: &Headers) -> Vec<u8> {
    let file = File::open(format!("{}{}", ROOT_DIR, &request_header.uri));
    let (mut body, status) = match file {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            (buffer, HttpStatus::from_usize(200))
        },
        Err(e) => {
            println!("{}", e);
            let mut f = File::open(format!("{}{}", ROOT_DIR, "/404.html")).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            (buffer, HttpStatus::from_usize(404))
        },
    };

    let mut send_buffer = [
        format!("HTTP/1.1 {}", status.to_string()),
        format!("Date:  {}", Local::now()),
        format!("Server: Modoki/0.1"),
        format!("Connection: close"),
        format!("{}", request_header.content_type.to_string()),
        format!(""),
        format!(""),
    ].join("\r\n").as_bytes().to_vec();

    send_buffer.append(&mut body);
    send_buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_extract_headers() {
        let resource = extract_headers(r#"GET /index.html HTTP/1.1
            Host: localhost:8000"#.to_string());
        assert_eq!(resource.method, Method::GET);
        assert_eq!(resource.uri, "/index.html".to_string());
        assert_eq!(resource.protcol, Protcol::HTTP1);
    }
}