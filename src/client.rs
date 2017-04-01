use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Client<'a> {
    host: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(host: &'a str) -> Self {
        Client { host: host }
    }

    pub fn send(&self) {
        let mut connection = TcpStream::connect(self.host).unwrap();

        let headers = "GET / HTTP/1.1\r\n
            Host: localhost:8000\r\n
            Connection: keep-alive\r\n
            Cache-Control: max-age=0\r\n
            Upgrade-Insecure-Requests: 1\r\n
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.133 Safari/537.36\r\n
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\r\n
            Accept-Encoding: gzip, deflate, sdch, br\r\n
            Accept-Language: en-US,en;q=0.8,ja;q=0.6\r\n
        ";

        // let mut send_file = File::open("send_client.txt").unwrap();
        // let mut send_buffer = Vec::new();
        // send_file.read_to_end(&mut send_buffer).unwrap();
        // connection.write_all(&send_buffer).unwrap();

        connection.write_all(&headers.as_bytes()).unwrap();
        println!("[CLIENT]: Send message to server. {:?}", connection);

        let mut recieve_file = File::create("recieve_client.txt").unwrap();
        let mut recieve_buffer = Vec::new();

        connection.read_to_end(&mut recieve_buffer).unwrap();
        recieve_file.write_all(&recieve_buffer).unwrap();
        println!("[CLIENT]: Recieve message from server.");
    }
}

