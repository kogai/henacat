use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs::File;

#[derive(Debug)]
pub struct Client<'a> {
    host: &'a str,
}

impl <'a> Client <'a> {
    pub fn new(host: &'a str) -> Self {
        Client {
            host: host,
        }
    }

    pub fn send(&self) {
        let mut connection = TcpStream::connect(self.host).unwrap();
        let mut send_file = File::open("send_client.txt").unwrap();
        let mut send_buffer = Vec::new();
        send_file.read_to_end(&mut send_buffer).unwrap();
        connection.write_all(&send_buffer).unwrap();
        println!("[CLIENT]: Send message to server.");

        let mut recieve_file = File::create("recieve_client.txt").unwrap();
        let mut recieve_buffer = Vec::new();
        connection.read_to_end(&mut recieve_buffer).unwrap();
        let result_read = recieve_file.write_all(&recieve_buffer);
        // recieve_file.write_all(&recieve_buffer).unwrap();
        match result_read {
            Ok(x) => println!("{:?}", x),
            Err(e) => println!("{:?}", e),
        }
        println!("[CLIENT]: Recieve message from server.");
    }
}

