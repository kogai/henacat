use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client<'a> {
    host: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(host: &'a str) -> Self {
        Client { host: host }
    }

    pub fn send(&self) -> String {
        let mut connection = TcpStream::connect(self.host).unwrap();

        let headers = "GET /index.html HTTP/1.1\r\n
            Host: localhost:8000\r\n
            Connection: keep-alive\r\n
            Cache-Control: max-age=0\r\n
            Upgrade-Insecure-Requests: 1\r\n
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.133 Safari/537.36\r\n
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\r\n
            Accept-Encoding: gzip, deflate, sdch, br\r\n
            Accept-Language: en-US,en;q=0.8,ja;q=0.6\r\n\r\n
        ";

        connection.write_all(&headers.as_bytes()).unwrap();

        let mut buffer = Vec::new();
        connection.read_to_end(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

