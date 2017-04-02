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
                        let response = create_response(&request_headers);
                        println!("{:?}", request_headers); // TODO: リクエストからパスを引き出す
                        stream.write_all(response.as_bytes()).unwrap();
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

#[derive(Debug)]
enum ContentType {
    TextHtml,
    TextCss,
    TextPlain,
    ImageJpg,
    ImagePng,
    ImageGif,
    ApplicationOctetStream,
}

impl ContentType {
    fn from_path(x: &str) -> Self {
        let extension = x.split(".").nth(1).unwrap_or("html").to_string();
        ContentType::from_string(extension)
    }

    fn from_string(x: String) -> Self {
        match x.as_str() {
            "html" | "htm" => ContentType::TextHtml,
            "css" => ContentType::TextCss,
            "jpg" | "jpeg" => ContentType::ImageJpg,
            "png" => ContentType::ImagePng,
            "gif" => ContentType::ImageGif,
            "txt" => ContentType::TextPlain,
            _ => ContentType::ApplicationOctetStream,
        }
    }

    fn to_string(&self) -> String {
        let content_type = match self {
            &ContentType::TextHtml => "text/html",
            &ContentType::TextCss => "text/css",
            &ContentType::TextPlain => "text/plain",
            &ContentType::ImageJpg => "image/jpeg",
            &ContentType::ImagePng => "image/png",
            &ContentType::ImageGif => "image/gif",
            _ => "application/octet-stream",
        };

        format!("Content-type: {}", content_type)
    }
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

fn create_response(request_header: &Headers) -> String {
    let send_buffer = [
        format!("HTTP/1.1 200 OK"),
        format!("Date:  {}", Local::now()),
        format!("Server: Modoki/0.1"),
        format!("Connection: close"),
        format!("{}", request_header.content_type.to_string()),
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
    fn it_should_extract_headers() {
        let resource = extract_headers(r#"GET /favicon.ico HTTP/1.1
            Host: localhost:8000"#.to_string());
        assert_eq!(resource.method, Method::GET);
        assert_eq!(resource.uri, "/favicon.ico".to_string());
        assert_eq!(resource.protcol, Protcol::HTTP1);
    }

    #[test]
    fn it_should_extract_content_type() {
        assert_eq!(ContentType::from_path("/test.png").to_string(), "Content-type: image/png");
        assert_eq!(ContentType::from_path("/test.jpg").to_string(), "Content-type: image/jpeg");
        assert_eq!(ContentType::from_path("/").to_string(), "Content-type: text/html");
    }
}