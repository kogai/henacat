use std::io::{Read, Write};
use std::net::TcpListener;
use std::fs::File;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str) -> Self {
        Server { listener: TcpListener::bind(host).unwrap() }
    }

    pub fn listen(&mut self) {
        println!("Waiting for connection from client...");

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("[SERVER]: Recieve connection from client. {:?}", stream);
                    let mut recieve_file = File::create("recieve_server.txt").unwrap();
                    let mut recieve_buffer = Vec::new();
                    stream.read_to_end(&mut recieve_buffer).unwrap();
                    recieve_file.write_all(&recieve_buffer).unwrap();

                    let mut send_file = File::open("send_server.txt").unwrap();
                    let mut send_buffer = Vec::new();
                    send_file.read_to_end(&mut send_buffer).unwrap();
                    stream.write_all(&send_buffer).unwrap();
                    println!("[SERVER]: Send message to client.");
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}
