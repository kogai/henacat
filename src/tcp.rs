use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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
            
                    // let mut send_file = File::open("send_server.txt").unwrap();
                    // let mut send_buffer = Vec::new();
                    // send_file.read_to_end(&mut send_buffer).unwrap();
                    // stream.write_all(&send_buffer).unwrap();
                    println!("[SERVER]: Send message to client.");
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Client {
  connection: TcpStream,
}

impl Client {
    pub fn new(host: &str) -> Self {
        Client {
            connection: TcpStream::connect(host).unwrap(),
        }
    }

    pub fn send(&mut self) {
        println!("[CLIENT]: Send message to server.");
        let mut send_file = File::open("send_client.txt").unwrap();
        let mut send_buffer = Vec::new();
        send_file.read_to_end(&mut send_buffer).unwrap();
        self.connection.write_all(&send_buffer).unwrap();

        // let mut recieve_file = File::create("recieve_client.txt").unwrap();
        // let mut recieve_buffer = Vec::new();
        // self.connection.read_to_end(&mut recieve_buffer).unwrap();
        // recieve_file.write_all(&recieve_buffer).unwrap();
        println!("[CLIENT]: Recieve message from server.");
    }
}

